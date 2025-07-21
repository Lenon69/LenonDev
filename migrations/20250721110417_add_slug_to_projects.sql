-- migrations/{timestamp}_add_slug_to_projects.sql
ALTER TABLE projects ADD COLUMN slug VARCHAR(255) UNIQUE;

-- Po dodaniu kolumny, musisz jednorazowo zaktualizować istniejące projekty.
-- Uruchom te komendy ręcznie w swojej bazie danych,
-- dostosowując 'nazwa-projektu-1' do swoich potrzeb.
-- UPDATE projects SET slug = 'nazwa-projektu-1' WHERE id = 1;
-- UPDATE projects SET slug = 'nazwa-projektu-2' WHERE id = 2;
-- ... i tak dalej dla każdego istniejącego projektu.
