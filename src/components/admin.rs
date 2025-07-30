use maud::{Markup, html};

use crate::models::{Article, Project, ProjectImage};

// Prosty layout dla panelu admina
pub fn admin_layout(title: &str, content: Markup) -> Markup {
    html! {
        // Używamy prostszej struktury niż na stronie głównej
        head {
            meta charset="UTF-8";
            title { (title) }
            script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {}
        }
        body class="bg-slate-800 text-white flex items-center justify-center min-h-screen" {
            (content)
        }
    }
}

// Formularz logowania
pub fn login_form() -> Markup {
    admin_layout(
        "Admin Login",
        html! {
            div class="w-full max-w-md bg-slate-900 p-8 rounded-lg shadow-lg" {
                h1 class="text-2xl font-bold text-center mb-6" { "Logowanie do panelu" }
                form action="/admin/login" method="post" class="space-y-4" {
                    div {
                        label for="password" class="block text-sm font-medium text-slate-300" { "Hasło" }
                        input type="password" name="password" id="password" required autocomplete="current-password"
                            class="mt-1 block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 focus:outline-none focus:ring-purple-500 focus:border-purple-500";
                    }
                    div {
                        button type="submit"
                            class="w-full bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded-lg transition-colors"
                        { "Zaloguj" }
                    }
                }
            }
        },
    )
}

/// Formularz dodawania artykułu
pub fn new_article_form() -> Markup {
    admin_layout(
        "Nowy Artykuł",
        html! {
            div class="w-full max-w-4xl bg-slate-900 p-8 rounded-lg shadow-lg" {
                h1 class="text-2xl font-bold mb-6" { "Utwórz nowy artykuł" }
                // Formularz będzie wysyłał dane metodą POST na nową trasę
                form action="/admin/articles/new" method="post" class="space-y-6" {
                    // Pole Tytuł
                    div {
                        label for="title" class="block text-sm font-medium text-slate-300 mb-1" { "Tytuł" }
                        input type="text" name="title" id="title" required
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 focus:outline-none focus:ring-purple-500 focus:border-purple-500";
                    }
                    // Pole Zajawka
                    div {
                        label for="excerpt" class="block text-sm font-medium text-slate-300 mb-1" { "Zajawka (krótki opis)" }
                        input type="text" name="excerpt" id="excerpt"
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 focus:outline-none focus:ring-purple-500 focus:border-purple-500";
                    }
                    // Pole Treść (jako textarea)
                    div {
                        label for="content" class="block text-sm font-medium text-slate-300 mb-1" { "Treść (HTML + TailwindCSS4)" }
                        textarea name="content" id="content" rows="12" required
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 font-mono focus:outline-none focus:ring-purple-500 focus:border-purple-500" {}
                    }
                    // Przyciski
                    div class="flex justify-end gap-4" {
                        a href="/admin/dashboard" class="bg-slate-700 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded-lg transition-colors" { "Anuluj" }
                        button type="submit"
                            class="bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded-lg transition-colors"
                        { "Opublikuj artykuł" }
                    }
                }
            }
        },
    )
}

/// Generuje widok głównego panelu admina z listą projektów i artykułów.
pub fn dashboard_view(
    articles: Vec<Article>,
    projects: Vec<Project>,
    current_page: i64,
    total_pages: i64,
) -> Markup {
    admin_layout(
        "Admin Dashboard",
        html! {
            div class="w-full max-w-6xl bg-slate-900 p-8 rounded-lg shadow-lg" {
                // Nagłówek panelu
                div class="flex justify-between items-center mb-8 border-b border-slate-700 pb-4" {
                    h1 class="text-3xl font-bold text-slate-100" { "Dashboard" }
                    div class="flex items-center gap-4" {
                        a href="/admin/projects/new" class="inline-block bg-brand-cyan hover:opacity-80 text-brand-dark font-bold py-2 px-4 rounded-lg transition-opacity" {
                            "🚀 Dodaj nowy projekt"
                        }
                        a href="/admin/articles/new" class="inline-block bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded-lg transition-colors" {
                            "✍️ Dodaj nowy artykuł"
                        }
                        a href="/admin/logout" class="inline-block bg-slate-700 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded-lg transition-colors" {
                            "Wyloguj"
                        }
                    }
                }

                // --- SEKCJA PROJEKTÓW ---
                h2 class="text-2xl font-bold text-slate-200 mt-8 mb-4" { "Zarządzaj Projektami" }
                div class="overflow-x-auto" {
                    table class="min-w-full text-left text-sm" {
                        thead class="border-b border-slate-700 text-slate-400" {
                            tr {
                                th scope="col" class="px-6 py-3" { "Tytuł Projektu" }
                                th scope="col" class="px-6 py-3" { "Technologie" }
                                th scope="col" class="px-6 py-3 text-right" { "Akcje" }
                            }
                        }
                        tbody {
                            @if projects.is_empty() {
                                tr {
                                    td colspan="3" class="px-6 py-4 text-center text-slate-500" {
                                        "Nie dodałeś jeszcze żadnych projektów."
                                    }
                                }
                            }
                            @for project in projects {
                                tr class="border-b border-slate-800 hover:bg-slate-800/50" {
                                    td class="px-6 py-4 font-medium text-slate-200" { (project.title) }
                                    td class="px-6 py-4 text-slate-400" { (project.technologies) }
                                    td class="px-6 py-4 text-right space-x-4" {
                                        a href=(format!("/admin/projects/edit/{}", project.id)) class="font-medium text-brand-cyan hover:underline" { "Edytuj" }
                                        // W przyszłości można tu dodać przycisk "Usuń"
                                    }
                                }
                            }
                        }
                    }
                }

                // --- SEKCJA ARTYKUŁÓW ---
                h2 class="text-2xl font-bold text-slate-200 mt-12 mb-4" { "Zarządzaj Artykułami" }
                div class="overflow-x-auto" {
                    table class="min-w-full text-left text-sm" {
                        thead class="border-b border-slate-700 text-slate-400" {
                            tr {
                                th scope="col" class="px-6 py-3" { "Tytuł Artykułu" }
                                th scope="col" class="px-6 py-3" { "Data publikacji" }
                                th scope="col" class="px-6 py-3 text-right" { "Akcje" }
                            }
                        }
                        tbody {
                            @if articles.is_empty() {
                                tr {
                                    td colspan="3" class="px-6 py-4 text-center text-slate-500" {
                                        "Nie ma jeszcze żadnych artykułów."
                                    }
                                }
                            }
                            @for article in articles {
                                tr class="border-b border-slate-800 hover:bg-slate-800/50" {
                                    td class="px-6 py-4 font-medium text-slate-200" { (article.title) }
                                    td class="px-6 py-4 text-slate-400" {
                                        @if let Some(published_at) = article.published_at {
                                            (published_at.format("%d-%m-%Y %H:%M"))
                                        } @else {
                                            span class="italic text-slate-500" { "Nieopublikowany" }
                                        }
                                    }
                                    td class="px-6 py-4 text-right space-x-4" {
                                        a href=(format!("/admin/articles/edit/{}", article.id)) class="font-medium text-brand-cyan hover:underline" { "Edytuj" }
                                        form action=(format!("/admin/articles/delete/{}", article.id)) method="post" class="inline" onsubmit="return confirm('Czy na pewno chcesz usunąć ten artykuł?');" {
                                            button type="submit" class="font-medium text-red-500 hover:underline" { "Usuń" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // --- NOWA SEKCJA: PAGINACJA ARTYKUŁÓW ---
                @if total_pages > 1 {
                    div class="flex justify-between items-center mt-6" {
                        // Przycisk "Poprzednia"
                        @if current_page > 1 {
                            a href=(format!("/admin/dashboard?page={}", current_page - 1))
                              class="inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-4 rounded-lg" {
                                "← Poprzednia"
                            }
                        } @else {
                            // Pusty element dla zachowania layoutu
                            span {}
                        }

                        // Informacja o stronie
                        span class="text-slate-400" {
                            "Strona " (current_page) " z " (total_pages)
                        }

                        // Przycisk "Następna"
                        @if current_page < total_pages {
                            a href=(format!("/admin/dashboard?page={}", current_page + 1))
                              class="inline-block bg-slate-700 hover:bg-slate-600 transition-colors text-white font-bold py-2 px-4 rounded-lg" {
                                "Następna →"
                            }
                        } @else {
                            span {}
                        }
                    }
                }
            }
        },
    )
}

/// Generuje widok formularza do edycji istniejącego artykułu.
pub fn edit_article_form(article: &Article) -> Markup {
    admin_layout(
        "Edytuj Artykuł",
        html! {
            div class="w-full max-w-4xl bg-slate-900 p-8 rounded-lg shadow-lg" {
                h1 class="text-2xl font-bold mb-6" { "Edytuj artykuł" }
                // Formularz będzie wysyłał dane metodą POST na trasę edycji
                form action=(format!("/admin/articles/edit/{}", article.id)) method="post" class="space-y-6" {
                    // Pole Tytuł
                    div {
                        label for="title" class="block text-sm font-medium text-slate-300 mb-1" { "Tytuł" }
                        input type="text" name="title" id="title" required value=(article.title)
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 focus:outline-none focus:ring-purple-500 focus:border-purple-500";
                    }
                    // Pole Zajawka
                    div {
                        label for="excerpt" class="block text-sm font-medium text-slate-300 mb-1" { "Zajawka (krótki opis)" }
                        input type="text" name="excerpt" id="excerpt" value=(article.excerpt.as_deref().unwrap_or(""))
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 focus:outline-none focus:ring-purple-500 focus:border-purple-500";
                    }
                    // Pole Treść (jako textarea)
                    div {
                        label for="content" class="block text-sm font-medium text-slate-300 mb-1" { "Treść (HTML + TailwindCSS4)" }
                        textarea name="content" id="content" rows="12" required
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 font-mono focus:outline-none focus:ring-purple-500 focus:border-purple-500" {
                            (article.content)
                        }
                    }
                    // Przyciski
                    div class="flex justify-end gap-4" {
                        a href="/admin/dashboard" class="bg-slate-700 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded-lg transition-colors" { "Anuluj" }
                        button type="submit"
                            class="bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded-lg transition-colors"
                        { "Zapisz zmiany" }
                    }
                }
            }
        },
    )
}

/// Formularz dodawania nowego projektu
pub fn new_project_form() -> Markup {
    admin_layout(
        "Nowy Projekt",
        html! {
            div class="w-full max-w-4xl bg-slate-900 p-8 rounded-lg shadow-lg" {
                h1 class="text-2xl font-bold mb-6" { "Dodaj nowy projekt" }
                form action="/admin/projects/new" method="post" class="space-y-6" {
                    // Pole Tytuł
                    div {
                        label for="title" class="block text-sm font-medium text-slate-300 mb-1" { "Tytuł projektu" }
                        input type="text" name="title" id="title" required
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 focus:outline-none focus:ring-purple-500 focus:border-purple-500";
                    }
                    // Pole Opis (jako textarea)
                    div {
                        label for="description" class="block text-sm font-medium text-slate-300 mb-1" { "Opis projektu" }
                        textarea name="description" id="description" rows="6" required
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 font-mono focus:outline-none focus:ring-purple-500 focus:border-purple-500" {}
                    }
                    // Pole Technologie
                    div {
                        label for="technologies" class="block text-sm font-medium text-slate-300 mb-1" { "Technologie (oddzielone przecinkami)" }
                        input type="text" name="technologies" id="technologies" required
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 focus:outline-none focus:ring-purple-500 focus:border-purple-500";
                    }
                    // Pole Główne zdjęcie
                    div {
                        label for="image_url" class="block text-sm font-medium text-slate-300 mb-1" { "URL głównego zdjęcia (np. /public/projects/nowy.avif)" }
                        input type="text" name="image_url" id="image_url"
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 focus:outline-none focus:ring-purple-500 focus:border-purple-500";
                    }
                    // Pole Link do projektu
                    div {
                        label for="project_url" class="block text-sm font-medium text-slate-300 mb-1" { "URL projektu (jeśli istnieje)" }
                        input type="text" name="project_url" id="project_url"
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 focus:outline-none focus:ring-purple-500 focus:border-purple-500";
                    }
                    // Przyciski
                    div class="flex justify-end gap-4" {
                        a href="/admin/dashboard" class="bg-slate-700 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded-lg transition-colors" { "Anuluj" }
                        button type="submit" class="bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded-lg transition-colors" { "Dodaj projekt" }
                    }
                }
            }
        },
    )
}

/// Generuje widok formularza do edycji istniejącego projektu.
/// włącznie z zarządzaniem zdjęciami.
pub fn edit_project_form(project: &Project, images: &[ProjectImage]) -> Markup {
    admin_layout(
        "Edytuj Projekt",
        html! {
            div class="w-full max-w-5xl bg-slate-900 p-8 rounded-lg shadow-lg" {
                // Nagłówek główny strony
                h1 class="text-2xl font-bold mb-6 border-b border-slate-700 pb-4" {
                    "Edytuj projekt: "
                    span class="text-brand-cyan" { (project.title) }
                }

                // --- SEKCJA 1: Formularz do edycji danych tekstowych ---
                h2 class="text-xl font-semibold text-slate-300 mb-4" { "Dane podstawowe" }
                form action=(format!("/admin/projects/edit/{}", project.id)) method="post" class="space-y-6 mb-12" {
                    // Tytuł
                    div {
                        label for="title" class="block text-sm font-medium text-slate-300 mb-1" { "Tytuł projektu" }
                        input type="text" name="title" id="title" required value=(project.title)
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 focus:outline-none focus:ring-purple-500 focus:border-purple-500";
                    }
                    // Opis (jako textarea)
                    div {
                        label for="description" class="block text-sm font-medium text-slate-300 mb-1" { "Opis (obsługuje HTML)" }
                        textarea name="description" id="description" rows="8" required
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 font-mono focus:outline-none focus:ring-purple-500 focus:border-purple-500" {
                            (project.description)
                        }
                    }
                    // Technologie
                    div {
                        label for="technologies" class="block text-sm font-medium text-slate-300 mb-1" { "Technologie (oddzielone przecinkami)" }
                        input type="text" name="technologies" id="technologies" required value=(project.technologies)
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 focus:outline-none focus:ring-purple-500 focus:border-purple-500";
                    }
                    // URL Głównego zdjęcia
                    div {
                        label for="image_url" class="block text-sm font-medium text-slate-300 mb-1" { "URL głównego zdjęcia (np. /public/projects/projekt.avif)" }
                        input type="text" name="image_url" id="image_url" value=(project.image_url.as_deref().unwrap_or(""))
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 focus:outline-none focus:ring-purple-500 focus:border-purple-500";
                    }
                    // URL projektu
                    div {
                        label for="project_url" class="block text-sm font-medium text-slate-300 mb-1" { "URL projektu (jeśli istnieje, np. https://...)" }
                        input type="text" name="project_url" id="project_url" value=(project.project_url.as_deref().unwrap_or(""))
                            class="block w-full bg-slate-800 border border-slate-700 rounded-md py-2 px-3 focus:outline-none focus:ring-purple-500 focus:border-purple-500";
                    }
                    // Przyciski
                    div class="flex justify-end gap-4 mt-4" {
                        a href="/admin/dashboard" class="bg-slate-700 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded-lg transition-colors" { "Powrót do dashboardu" }
                        button type="submit" class="bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded-lg transition-colors" { "Zapisz zmiany" }
                    }
                }

                // --- SEKCJA 2: Zarządzanie zdjęciami ---
                div class="border-t border-slate-700 pt-8" {
                    h2 class="text-xl font-semibold text-slate-300 mb-4" { "Zdjęcia w galerii" }

                    // Formularz do wgrywania nowych zdjęć
                    form action=(format!("/admin/projects/add-image/{}", project.id)) method="post" enctype="multipart/form-data" class="bg-slate-800/50 p-4 rounded-lg flex items-center gap-4 mb-8 border border-slate-700" {
                        div class="flex-grow" {
                            label for="image_upload" class="sr-only" { "Wgraj nowe zdjęcie" }
                            input type="file" name="image" id="image_upload" required
                                class="block w-full text-sm text-slate-400 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-purple-600 file:text-white hover:file:bg-purple-700 cursor-pointer";
                        }
                        button type="submit" class="bg-brand-cyan hover:opacity-80 text-brand-dark font-bold py-2 px-4 rounded-full transition-opacity flex-shrink-0" { "Dodaj zdjęcie" }
                    }

                    // Lista istniejących zdjęć
                    @if images.is_empty() {
                        p class="text-slate-500 text-center py-4" { "Ten projekt nie ma jeszcze dodatkowych zdjęć w galerii." }
                    } @else {
                        div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-4" {
                            @for image in images {
                                div class="relative group" {
                                    img src=(image.image_url) alt="Zdjęcie projektu" class="rounded-lg object-cover w-full h-32 border-2 border-slate-700";
                                    // Formularz usuwania owija przycisk
                                    form action=(format!("/admin/projects/delete-image/{}", image.id)) method="post" class="absolute top-2 right-2" onsubmit="return confirm('Czy na pewno chcesz usunąć to zdjęcie?');" {
                                        button type="submit" class="bg-red-600/70 hover:bg-red-600 text-white rounded-full p-1.5 opacity-0 group-hover:opacity-100 transition-all duration-300" aria-label="Usuń zdjęcie" {
                                            // Ikonka kosza
                                            svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor" {
                                                path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm4 0a1 1 0 012 0v6a1 1 0 11-2 0V8z" clip-rule="evenodd" {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
    )
}
