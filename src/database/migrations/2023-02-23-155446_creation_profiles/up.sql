CREATE TABLE IF NOT EXISTS profiles (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  profile_token VARCHAR(60) NOT NULL,
  name VARCHAR(30) NOT NULL,
  surname VARCHAR(60) NOT NULL,
  email VARCHAR(100) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('profiles');
INSERT INTO profiles (user_id, profile_token, name, surname, email) VALUES
  (1, 'admin',  'admin',  'admin',        'admin@admin.es'),
  (2, 'coord1', 'coord1', 'coordinator',  'coord1@coordinator.es'),
  (3, 'coord2', 'coord2', 'coordinator',  'coord2@coordinator.es'),
  (4, 'thera1', 'thera1', 'therapist',    'thera1@therapist.es'),
  (5, 'thera2', 'thera2', 'therapist',    'thera2@therapist.es'),
  (6, 'thera3', 'thera3', 'therapist',    'thera3@therapist.es'),
  (7, 'user1',  'user1',  'patient',      'user1@patient.es'),
  (8, 'user2',  'user2',  'patient',      'user2@patient.es'),
  (9, 'user3',  'user3',  'patient',      'user3@patient.es'),
 (10, 'user4',  'user4',  'patient',      'user4@patient.es'),
 (11, 'user5',  'user5',  'patient',      'user5@patient.es') ON CONFLICT DO NOTHING
  ;

-- update the sequence to the max id
-- that prevents in case of a conflict to have a duplicate id
SELECT setval('profiles_id_seq', (SELECT MAX(id) FROM profiles));
