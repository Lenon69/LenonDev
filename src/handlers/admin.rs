use std::{fs, path};

// src/handlers/admin.rs
use crate::{
    AppState,
    components::{self},
    models::{Article, Project, ProjectForm, ProjectImage, ProjectUpdateForm},
};
use axum::{
    Form, Router,
    extract::{DefaultBodyLimit, Path, Request, State},
    middleware::Next,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
use maud::Markup;
use serde::Deserialize;
use slug; // Upewnij się, że ten import jest
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct LoginForm {
    password: String,
}

#[derive(Deserialize)]
struct ArticleForm {
    title: String,
    excerpt: String,
    content: String,
}

// === HANDLERY ===

async fn get_login_form() -> Html<maud::Markup> {
    Html(components::admin::login_form())
}

async fn post_login(session: Session, Form(form): Form<LoginForm>) -> impl IntoResponse {
    // Odczytujemy hasło ze zmiennej środowiskowej (z pliku .env)
    let admin_password =
        std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD musi być ustawione w pliku .env");

    // Porównujemy hasło z formularza z tym z .env
    if form.password == admin_password {
        // Hasło poprawne - logujemy, wstawiając do sesji prosty identyfikator
        session.insert("user_id", "admin").await.unwrap();
        Redirect::to("/admin/dashboard")
    } else {
        // Hasło niepoprawne
        Redirect::to("/admin/login")
    }
}

async fn logout(session: Session) -> impl IntoResponse {
    session.clear().await;
    Redirect::to("/")
}

async fn dashboard(State(state): State<AppState>) -> Html<maud::Markup> {
    // Pobieramy artykuły (bez zmian)
    let articles = sqlx::query_as::<_, Article>("SELECT * FROM articles ORDER BY created_at DESC")
        .fetch_all(&state.db_pool)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Błąd podczas pobierania artykułów: {}", e);
            vec![]
        });

    // DODAJ TO: Pobieramy projekty
    let projects = sqlx::query_as::<_, crate::models::Project>(
        "SELECT id, title, slug, description, technologies, image_url, project_url FROM projects ORDER BY id DESC"
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_else(|e| {
        eprintln!("Błąd podczas pobierania projektów: {}", e);
        vec![]
    });

    // Przekazujemy oba zestawy danych do widoku
    Html(components::admin::dashboard_view(articles, projects))
}

// Handler GET /admin/articles/new
async fn get_new_article_form() -> Html<maud::Markup> {
    Html(components::admin::new_article_form())
}

// === POPRAWIONY HANDLER POST ===
// Handler dodający nowy artykuł
async fn post_new_article(
    State(state): State<AppState>,
    Form(form): Form<ArticleForm>,
) -> impl IntoResponse {
    let slug = slug::slugify(&form.title);

    let query_result = sqlx::query(
        "INSERT INTO articles (title, slug, excerpt, content, published_at) VALUES ($1, $2, $3, $4, NOW())",
    )
    .bind(form.title)
    .bind(slug)
    .bind(form.excerpt)
    .bind(form.content)
    .execute(&state.db_pool)
    .await;

    match query_result {
        Ok(_) => {
            // POPRAWKA: Unieważniamy cache dla pierwszej strony bloga
            let cache_key = "page:/blog?page=1";
            println!("CACHE INVALIDATION: Unieważniono klucz: {}", cache_key);
            state.cache.invalidate(cache_key);
            Ok(Redirect::to("/admin/dashboard"))
        }
        Err(e) => {
            eprintln!("Błąd zapisu do bazy danych: {}", e);
            Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Błąd serwera podczas zapisu artykułu.",
            ))
        }
    }
}

// === MIDDLEWARE ("STRAŻNIK") ===
pub async fn auth_middleware(
    session: Session,
    State(_state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let is_logged_in = session
        .get::<String>("user_id")
        .await
        .map(|user| user.is_some())
        .unwrap_or(false);
    if is_logged_in {
        next.run(request).await
    } else {
        Redirect::to("/admin/login").into_response()
    }
}

// === FUNKCJE TWORZĄCE ROUTERY ===
pub fn public_admin_routes() -> Router<AppState> {
    Router::new().route("/login", get(get_login_form).post(post_login))
}

pub fn protected_admin_routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(dashboard))
        .route("/logout", get(logout))
        .route(
            "/articles/new",
            get(get_new_article_form).post(post_new_article),
        )
        .route(
            "/articles/edit/{id}",
            get(get_edit_article_form).post(post_update_article),
        )
        .route("/articles/delete/{id}", post(post_delete_article))
        .route(
            "/projects/new",
            get(get_new_project_form).post(post_new_project),
        )
        .route(
            "/projects/edit/{id}",
            get(get_edit_project_form).post(post_update_project),
        )
        .route("/projects/add-image/{id}", post(post_add_project_image))
        .route(
            "/projects/delete-image/{id}",
            post(post_delete_project_image),
        )
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // Ustaw limit 10MB na wgrywane pliki
}

// Handler GET /admin/articles/edit/{id} - wyświetla formularz edycji
async fn get_edit_article_form(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Markup, Redirect> {
    match sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db_pool)
        .await
    {
        Ok(article) => Ok(components::admin::edit_article_form(&article)),
        Err(_) => Err(Redirect::to("/admin/dashboard")), // Jeśli nie ma artykułu, przekieruj
    }
}

// Handler POST /admin/articles/edit/{id} - zapisuje zmiany
async fn post_update_article(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(form): Form<ArticleForm>,
) -> impl IntoResponse {
    let slug = slug::slugify(&form.title);

    let query_result = sqlx::query(
        "UPDATE articles SET title = $1, slug = $2, excerpt = $3, content = $4, updated_at = NOW() WHERE id = $5",
    )
    .bind(form.title)
    .bind(&slug) // Przekazujemy referencję do sluga
    .bind(form.excerpt)
    .bind(form.content)
    .bind(id)
    .execute(&state.db_pool)
    .await;

    if query_result.is_ok() {
        // POPRAWKA: Unieważniamy cache dla pierwszej strony bloga ORAZ edytowanego artykułu
        let blog_cache_key = "page:/blog?page=1";
        let article_cache_key = format!("page:/blog/{}", slug);

        println!(
            "CACHE INVALIDATION: Unieważniono klucze: {}, {}",
            blog_cache_key, article_cache_key
        );
        state.cache.invalidate(blog_cache_key);
        state.cache.invalidate(&article_cache_key);
    }

    Redirect::to("/admin/dashboard")
}

// Handler POST /admin/articles/delete/{id} - usuwa artykuł
async fn post_delete_article(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    // KROK 3: Najpierw pobieramy slug artykułu, ZANIM go usuniemy
    let article_slug: Option<String> =
        sqlx::query_scalar("SELECT slug FROM articles WHERE id = $1")
            .bind(id)
            .fetch_one(&state.db_pool)
            .await
            .ok();

    // Usuwamy artykuł z bazy danych
    sqlx::query("DELETE FROM articles WHERE id = $1")
        .bind(id)
        .execute(&state.db_pool)
        .await
        .ok();

    // POPRAWKA: Unieważniamy cache dla pierwszej strony bloga
    let blog_cache_key = "page:/blog?page=1";
    println!("CACHE INVALIDATION: Unieważniono klucz: {}", blog_cache_key);
    state.cache.invalidate(blog_cache_key);

    // Jeśli udało się pobrać slug, unieważniamy także cache dla tego konkretnego artykułu
    if let Some(slug) = article_slug {
        println!(
            "CACHE INVALIDATION: Artykuł /blog/{} został unieważniony.",
            slug
        );
        state.cache.invalidate(&format!("page:/blog/{}", slug));
    }

    Redirect::to("/admin/dashboard")
}

// Handler GET /admin/projects/new - wyświetla formularz
async fn get_new_project_form() -> Html<Markup> {
    Html(components::admin::new_project_form())
}

// Handler POST /admin/projects/new - zapisuje nowy projekt
async fn post_new_project(
    State(state): State<AppState>,
    Form(form): Form<ProjectForm>,
) -> impl IntoResponse {
    let slug = slug::slugify(&form.title);

    let query_result = sqlx::query(
        "INSERT INTO projects (title, slug, description, technologies, image_url, project_url) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(form.title)
    .bind(slug)
    .bind(form.description)
    .bind(form.technologies)
    .bind(form.image_url)
    .bind(form.project_url)
    .execute(&state.db_pool)
    .await;

    match query_result {
        Ok(_) => {
            // Unieważniamy cache strony głównej, aby nowy projekt był widoczny
            println!(
                "CACHE INVALIDATION: Strona główna została unieważniona z powodu nowego projektu."
            );
            state.cache.invalidate("page:/:scroll_to=");
            Ok(Redirect::to("/admin/dashboard"))
        }
        Err(e) => {
            eprintln!("Błąd zapisu projektu do bazy danych: {}", e);
            Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Błąd serwera podczas zapisu projektu.",
            ))
        }
    }
}

// GET /admin/projects/edit/:id - Wyświetla stronę edycji
async fn get_edit_project_form(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Markup, Redirect> {
    let project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| Redirect::to("/admin/dashboard"))?;

    let images = sqlx::query_as::<_, ProjectImage>(
        "SELECT id, image_url FROM project_images WHERE project_id = $1 ORDER BY id",
    )
    .bind(id)
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_else(|_| vec![]);

    Ok(components::admin::edit_project_form(&project, &images))
}

// POST /admin/projects/edit/:id - Aktualizuje dane tekstowe
async fn post_update_project(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(form): Form<ProjectUpdateForm>,
) -> impl IntoResponse {
    let slug = slug::slugify(&form.title);

    sqlx::query(
        "UPDATE projects SET title = $1, slug = $2, description = $3, technologies = $4, image_url = $5, project_url = $6 WHERE id = $7",
    )
    .bind(form.title).bind(&slug).bind(form.description).bind(form.technologies).bind(form.image_url).bind(form.project_url).bind(id)
    .execute(&state.db_pool).await.ok();

    // Unieważnij cache dla strony głównej i edytowanego projektu
    state.cache.invalidate("page:/:scroll_to=");
    state
        .cache
        .invalidate(&format!("fragment:/projekty/{}", slug));

    Redirect::to(&format!("/admin/projects/edit/{}", id))
}

// POST /admin/projects/add-image/:id - Wgrywa nowe zdjęcie
async fn post_add_project_image(
    State(state): State<AppState>,
    Path(project_id): Path<i32>,
    mut multipart: axum_extra::extract::Multipart,
) -> Redirect {
    // Pobierz aktualną liczbę zdjęć, aby wygenerować unikalną nazwę
    let image_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM project_images WHERE project_id = $1")
            .bind(project_id)
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or(0);

    let file_name = format!("{}-{}.avif", project_id, image_count + 1);
    let file_path = path::Path::new("static/projects").join(&file_name);
    let image_url = format!("/public/projects/{}", file_name);

    if let Some(field) = multipart.next_field().await.unwrap() {
        if let Ok(data) = field.bytes().await {
            fs::write(&file_path, data).unwrap();

            // Zapisz ścieżkę do bazy danych
            sqlx::query("INSERT INTO project_images (project_id, image_url) VALUES ($1, $2)")
                .bind(project_id)
                .bind(&image_url)
                .execute(&state.db_pool)
                .await
                .ok();

            // Unieważnij cache dla tego projektu
            if let Ok(slug_result) =
                sqlx::query_scalar::<_, String>("SELECT slug from projects WHERE id = $1")
                    .bind(project_id)
                    .fetch_one(&state.db_pool)
                    .await
            {
                state
                    .cache
                    .invalidate(&format!("fragment:/projekty/{}", slug_result));
            }
        }
    }

    Redirect::to(&format!("/admin/projects/edit/{}", project_id))
}

// Handler POST /admin/projects/delete-image/:id - Usuwa zdjęcie
async fn post_delete_project_image(
    State(state): State<AppState>,
    Path(image_id): Path<i32>,
) -> Redirect {
    // Krok 1: Pobierz informacje o zdjęciu (URL i ID projektu) ZANIM je usuniesz
    let image_info = sqlx::query_as::<_, (String, i32)>(
        "SELECT image_url, project_id FROM project_images WHERE id = $1",
    )
    .bind(image_id)
    .fetch_one(&state.db_pool)
    .await;

    // Jeśli nie udało się znaleźć zdjęcia, po prostu przekieruj z powrotem
    let (image_url, project_id) = match image_info {
        Ok(info) => info,
        Err(_) => {
            // Jeśli projekt nie istnieje, przekieruj do dashboardu
            return Redirect::to("/admin/dashboard");
        }
    };

    // Krok 2: Usuń plik fizycznie z serwera
    // Przekształcamy URL (/public/projects/plik.png) na ścieżkę lokalną (static/projects/plik.png)
    let local_path = image_url.replace("/public/", "static/");
    match fs::remove_file(&local_path) {
        Ok(_) => println!("Pomyślnie usunięto plik: {}", local_path),
        Err(e) => eprintln!("Błąd podczas usuwania pliku {}: {}", local_path, e),
    }

    // Krok 3: Usuń wpis o zdjęciu z bazy danych
    sqlx::query("DELETE FROM project_images WHERE id = $1")
        .bind(image_id)
        .execute(&state.db_pool)
        .await
        .ok();

    // Krok 4: Unieważnij cache dla powiązanego projektu
    if let Ok(project_slug) =
        sqlx::query_scalar::<_, String>("SELECT slug FROM projects WHERE id = $1")
            .bind(project_id)
            .fetch_one(&state.db_pool)
            .await
    {
        let cache_key = format!("fragment:/projekty/{}", project_slug);
        state.cache.invalidate(&cache_key);
        println!(
            "CACHE INVALIDATION: Unieważniono cache dla klucza: {}",
            cache_key
        );
    }

    // Krok 5: Przekieruj użytkownika z powrotem na stronę edycji tego projektu
    Redirect::to(&format!("/admin/projects/edit/{}", project_id))
}
