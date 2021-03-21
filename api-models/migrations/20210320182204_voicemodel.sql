-- Add migration script here
ALTER TABLE guild
ADD voice_model TEXT NOT NULL
DEFAULT 'JP-Female-Normal-A'
