CREATE TABLE views (
    id serial NOT NULL,
    video_id integer NOT NULL,
    user_id integer NOT NULL,
    watch_start timestamp without time zone NOT NULL,
    duration integer NOT NULL,
    CONSTRAINT views_pkey PRIMARY KEY (id),
    CONSTRAINT views_video_id_fkey FOREIGN KEY (video_id)
        REFERENCES videos (id) MATCH SIMPLE
        ON UPDATE NO ACTION ON DELETE NO ACTION,
    CONSTRAINT views_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE NO ACTION ON DELETE NO ACTION
)