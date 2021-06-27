<script context="module">
	export async function load({ fetch }) {
		const url = 'http://localhost:8080/countries'
		const res = await fetch(url)

		if (res.ok) {
		  return {
				props: {
					countries: await res.json()
				}
			}
		}

		return {
			status: res.status,
			error: new Error(`Could not load ${url}`)
		}
	}

</script>

<script>
	export let countries;
</script>


<svelte:head>
				<title>Welcome</title>
</svelte:head>

<ul>
	{#each countries as country}
		<li>{country.name}</li>
	{/each}
</ul>


