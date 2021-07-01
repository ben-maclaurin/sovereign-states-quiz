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
		import { writable } from "svelte/store"
		export let countries;

		let correct = writable([]) 

		let response = '';
		let count = 0;
			let answer = ''


		const handleKeydown = (e) => {
			  if (e.keyCode == 13 && response.toLowerCase() === countries[count].name.toLowerCase()) {
						answer = ''
					response = ''
					$correct = [...$correct, countries[count].name];
					count++
				}
		}

			const showAnswer = () => {
					answer = `The correct answer was ${countries[count].name}`
					$correct = [] 
					count = 0
				}
</script>


<svelte:head>
	<title>Welcome</title>
</svelte:head>

<svelte:window on:keydown={handleKeydown} />

<div class="font-mono space-y-4 p-12">
	<p class="text-xl">learn countries a-z</p>
	<input on:input={() => onInput()} class="h-12 placeholder-gray-400 focus:outline-none" placeholder="enter country ..." bind:value={response} />
	<p on:click={() => showAnswer()} class="text-blue-700 cursor-pointer">show answer</p>
	<p>{answer}</p>
	<ul class="text-sm grid grid-cols-3">
		{#each $correct as country}
			<li>{country}</li>
		{/each}
	</ul>
</div>
