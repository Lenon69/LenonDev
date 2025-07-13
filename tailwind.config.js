/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],

  // --- NOWA SEKCJA ---
  // Ta opcja gwarantuje, że podane klasy zawsze znajdą się w finalnym pliku CSS,
  // omijając problem z ich ewentualnym nieodnalezieniem w plikach .rs
  safelist: ["bg-brand-dark", "text-slate-200", "text-slate-400"],
  // --- KONIEC NOWEJ SEKCJI ---

  theme: {
    extend: {
      colors: {
        "brand-dark": "#101014",
        "brand-purple": "#8B5CF6",
        "brand-cyan": "#2DD4BF",
        "brand-green": "#A3E635",
      },
      boxShadow: {
        "cyan-glow":
          "0 0 15px rgba(45, 212, 191, 0.4), 0 0 25px rgba(45, 212, 191, 0.1)",
      },
    },
  },
  plugins: [],
};
