use maud::{Markup, html};

use crate::models::Article;

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
                        label for="content" class="block text-sm font-medium text-slate-300 mb-1" { "Treść (Markdown)" }
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

/// Generuje widok głównego panelu admina z listą artykułów.
pub fn dashboard_view(articles: Vec<Article>) -> Markup {
    admin_layout(
        "Admin Dashboard",
        html! {
            div class="w-full max-w-6xl bg-slate-900 p-8 rounded-lg shadow-lg" {
                // Nagłówek panelu
                div class="flex justify-between items-center mb-8 border-b border-slate-700 pb-4" {
                    h1 class="text-3xl font-bold text-slate-100" { "Dashboard" }
                    div {
                        a href="/admin/projects/new" class="inline-block bg-brand-cyan hover:opacity-80 text-brand-dark font-bold py-2 px-4 rounded-lg transition-opacity mr-4" {
                        "🚀 Dodaj nowy projekt"
                        }
                        a href="/admin/articles/new" class="inline-block bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded-lg transition-colors" {
                            "✍️ Dodaj nowy artykuł"
                        }
                        a href="/admin/logout" class="inline-block bg-slate-700 hover:bg-slate-600 text-white font-bold py-2 px-4 rounded-lg transition-colors ml-4" {
                            "Wyloguj"
                        }
                    }
                }

                // Tabela z artykułami
                div class="overflow-x-auto" {
                    table class="min-w-full text-left text-sm" {
                        thead class="border-b border-slate-700 text-slate-400" {
                            tr {
                                th scope="col" class="px-6 py-3" { "Tytuł" }
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
                                        // Linki do edycji i usuwania (na razie nie działają)
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
                        label for="content" class="block text-sm font-medium text-slate-300 mb-1" { "Treść (Markdown)" }
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
                        label for="image_url" class="block text-sm font-medium text-slate-300 mb-1" { "URL głównego zdjęcia (np. /public/projects/nowy.png)" }
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
