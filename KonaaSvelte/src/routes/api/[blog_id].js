/** @type {import './[blog_id]'.RequestHandler } */
export async function get({ params }) {
    const { blog_id } = params;
    const res = await fetch(
        `http://127.0.0.1/api/${blog_id}`
    );
    const blog = await res.json();

    return { body: blog };    
}