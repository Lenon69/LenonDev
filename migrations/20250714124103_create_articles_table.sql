-- migrations/{TIMESTAMP}_create_articles_table.sql
CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    -- 'slug' to przyjazny dla SEO identyfikator w URL, np. /blog/moj-pierwszy-artykul
    slug VARCHAR(255) NOT NULL UNIQUE,
    -- 'excerpt' to krótki wstęp/zajawka widoczna na liście artykułów
    excerpt TEXT,
    -- 'content' to pełna treść artykułu, którą będziemy pisać w Markdown
    content TEXT NOT NULL,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
