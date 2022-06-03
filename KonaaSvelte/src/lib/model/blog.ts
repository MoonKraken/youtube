export type Blog = {
    title: string;
    subtitle: string;
    about: string;
    posts: Array<Post>;
    loading: boolean;
}

export type Post = {
    post_id: string;
    author: string;
    content: string;
    title: string;
}