<script lang="ts">
    import type { Blog } from '$lib/model/blog';
    import { page } from '$app/stores';
    import BlogHeader from '$lib/component/BlogHeader.svelte';
    import BlogPost from '$lib/component/BlogPost.svelte';
    import { onMount } from 'svelte';

    let blog_id = $page.params.blog_id;

    let blog: Blog = { loading: true, posts: [] };
    onMount(async () => {
        let res = await fetch(`/api/${blog_id}`)
        blog = await res.json();
        console.log(blog);
        blog.loading = false;
    });
</script>

<BlogHeader title={blog.title} subtitle={blog.subtitle}/>
{ #if blog.loading == false}
    { #each blog.posts as post }
        <div>
            <BlogPost {post}/>
        </div>
    {:else}
        <p>No posts</p>
    {/each}
{/if}

<style>
</style>