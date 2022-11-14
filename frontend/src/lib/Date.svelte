<script>
	export let value;

	const date = new Date(value);
</script>

{date.getDate()}-{date.getMonth()}-{date.getFullYear()}
