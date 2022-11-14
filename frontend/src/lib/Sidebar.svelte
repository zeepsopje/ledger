<script>
	export let navItems = [];
</script>

<div class="sidebar">
	<nav>
		{#each navItems as {label, url}}
			<a href={url}>{label}</a>
		{/each}
	</nav>
</div>

<style>
	.sidebar {
		width: 250px;
		height: 100%;
		border-right: 1px solid #000;
	}
</style>
