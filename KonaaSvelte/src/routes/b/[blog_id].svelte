<script lang="ts">
    import { page } from '$app/stores';
    import BlogHeader from '$lib/component/BlogHeader.svelte';
    import BlogPost from '$lib/component/BlogPost.svelte';
    import { onMount } from 'svelte';

    let blog_id = $page.params.blog_id;

    let blog = { loading: true, posts: [] };
    onMount(async () => {
        //const res = await fetch(`/api/${blog_id}`)
        //blog = await res.json();
        blog = {
            title: "Test title",
            subtitle: "Test subtitle",
            posts: [
                {
                    datetime: "2022-05-12T21:24:49Z",
                    title: "First post",
                    content: "SvelteKit is a framework for building extremely high-performance web apps.  Building an app with all the modern best practices is fiendishly complicated. Those practices include build optimizations, so that you load only the minimal required code; offline support; prefetching pages before the user initiates navigation; and configurable rendering that allows you to generate HTML on the server or in the browser at runtime or at build-time. SvelteKit does all the boring stuff for you so that you can get on with the creative part.  It uses Vite with a Svelte plugin to provide a lightning-fast and feature-rich development experience with Hot Module Replacement (HMR), where changes to your code are reflected in the browser instantly.  You don't need to know Svelte to understand the rest of this guide, but it will help. In short, it's a UI framework that compiles your components to highly optimized vanilla JavaScript. Read the introduction to Svelte blog post and the Svelte tutorial to learn more.",
                    author: "Ken"
                }
            ],
            nextToken: "asdf",
            loading: false,
        };
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