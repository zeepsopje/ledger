<script>
	import "../app.scss";
	import Sidebar from "../lib/Sidebar.svelte";

	const navItems = [
		{
			label: "Dashboard",
			url: "/",
		},
	];
</script>

<div class="root">
	<div class="sidebar">
		<Sidebar {navItems} />
	</div>
	<div class="view">
		<slot />
	</div>
</div>

<style>
	.root {
		display: flex;
	}

	.sidebar {
		height: 100vh;
	}
</style>
