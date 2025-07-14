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
