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

		let response = '';
		let count = 0;

			const handleKeydown = (e) => {
					let keyCode = e.keyCode;

					if (e.keyCode === 13 && response.toLowerCase() === countries[count].name.toLowerCase()) {
							response = ''
							count++
						}
				}

</script>


<svelte:head>
	<title>Welcome</title>
</svelte:head>

<svelte:window on:keydown={handleKeydown} />

<input bind:value={response} />
<p>{count}</p>
