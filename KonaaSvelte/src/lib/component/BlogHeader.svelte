<script lang="ts">
    export let title: string;
    export let subtitle: string;
</script>

<nav>
<h1>{title}</h1>
<h3>{subtitle}</h3>
</nav>

<style>
    nav {
        display: flex;
        align-items: baseline;
    }

    h1 {
    }

    h3 {
    }
</style>