use maud::{Markup, html};

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
                        input type="password" name="password" id="password" required
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
