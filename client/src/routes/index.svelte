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

		let country = countries[0];
		let response = '';



</script>


<svelte:head>
	<title>Welcome</title>
</svelte:head>

<h1>{country.name}</h1>
<input bind:value={response} />
<p>{response}</p>
