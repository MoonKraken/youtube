<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import { signOut as authSignOut } from 'sk-auth/client';
	import { session } from '$app/stores';
	import Component from '$lib/component/Component.svelte';
    // getting the user from the session store
    $: user = $session.user;

	let some_var = {};

    function signIn() {
        location.assign('/api/auth/signin/cognito?redirect=/');
    }

    function signOut() {
        location.assign("/api/auth/signout");
		authSignOut().then(session.set);
	}
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1>
		<div class="welcome">
			<picture>
				<source srcset="svelte-welcome.webp" type="image/webp" />
				<img src="svelte-welcome.png" alt="Welcome" />
			</picture>
		</div>

		to your asdfnew<br />SvelteKit app
	</h1>

	<Component title={some_var.function()}/>
	<h2>
		try editing <strong>src/routes/index.svelte</strong>
	</h2>

	{#if !user}
    <button on:click="{signIn}">Log In with Cognito</button>
	{:else}
    <h2>Welcome {user.email}!</h2>
	<p>Your username is {user.username} and your email has been verified: {user.email_verified}</p>
    <button on:click={signOut}>Log Out</button>
	{/if}
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 1;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		display: block;
	}
</style>
