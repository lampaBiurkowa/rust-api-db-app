-- Your SQL goes here
ALTER TABLE blog_posts
ADD COLUMN category TEXT NOT NULL DEFAULT 'General';