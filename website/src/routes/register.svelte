<script>
	import { Circle } from 'svelte-loading-spinners';
	import passwordChecker from 'zxcvbn';
	import tippy from 'sveltejs-tippy';
	let feedback = '';
	let loggedInSuccessfully = false;
	let loginData = {
		password1: '',
		password2: '',
		email: ''
	};
	export let page;
	let valid = {
		password1: false,
		password2: false,
		email: false
	};

	let passwdstrength;
	let submitLoading = false;
	let modalMessage = '';
	let modalOpen = false;

	const login = async () => {
		submitLoading = true;
		console.log('login');
		const json_data = {
			email: loginData.email,
			password: loginData.password1
		};
		const res = await fetch(`/api/v1/users/create`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(json_data)
		});
		if (res.status === 200) {
			modalMessage =
				'You successfully created your account! You just have to confirm your email-address and install envwoman and you are ready to go!';
			modalOpen = true;
		} else {
			if (res.status === 400) {
				modalMessage = 'An account with this email-address already exists!';
				modalOpen = true;
			} else {
				modalMessage = 'The good old unexpected error occured! I hope you like it!';
				modalOpen = true;
			}
		}
		submitLoading = false;
	};

	const getFeedback = (password) => {
		return passwordChecker(password).feedback.warning;
	};
	const checkPassword = (password) => {
		const res = passwordChecker(password);
		return res.score >= 3;
	};

	$: passwdstrength = getFeedback(loginData.password1);
	$: valid.password2 = loginData.password1 === loginData.password2 && loginData.password2 !== '';
	$: valid.password1 = checkPassword(loginData.password1);
	let emailRegex = /^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6}$/;
	$: valid.email = emailRegex.exec(loginData.email) !== null;
</script>

<div class="">
	{#if loggedInSuccessfully}
		<div class="h-full absolute w-full bg-black opacity-80 z-10 grid place-items-center">
			<div>
				<Circle size="30" unit="rem" />
			</div>
		</div>
	{/if}
	<div class="h-screen flex">
		<div
			class="w-1/2 bg-gradient-to-tr from-mawoka-300 via-mawoka-200 to-mawoka-100 justify-around items-center hidden lg:flex"
		>
			<div>
				<h1 class="text-white font-bold text-4xl font-sans">Envwoman</h1>
				<p class="text-white mt-1">Your .env synchroniser!</p>
				<a
					href="/internal/about"
					class="block w-28 bg-white text-mawoka-300 mt-4 py-2 rounded-2xl font-bold mb-2 text-center"
					>Read More</a
				>
			</div>
		</div>
		<div class="flex lg:w-1/2 justify-center items-center bg-white w-screen">
			<form class="bg-white" on:submit|preventDefault={login}>
				<h1 class="text-gray-800 font-bold text-2xl mb-1">Nice to meet you!</h1>
				<p class="text-sm font-normal text-gray-600 mb-7">Just sync your .env's!</p>
				<div class="form-control">
					<label class="input-group input-group-vertical input-group-l text-black">
						<span>Email</span>
						<input
							class:input-error={!valid.email}
							type="text"
							placeholder="info@site.com"
							class="input input-bordered text-black"
							bind:value={loginData.email}
						/>
					</label>
					<label class="input-group input-group-vertical input-group-l pt-4 text-black">
						<span>Password</span>
						<input
							class:input-error={!valid.password1}
							type="password"
							placeholder="Your Password"
							class="input input-bordered text-black"
							use:tippy={{ content: "No, this thing isn't broken:D" }}
							bind:value={loginData.password1}
						/>
					</label>
					<p class="text-red-600 w-[15rem]">{passwdstrength}</p>

					<label class="input-group input-group-vertical input-group-l pt-4 text-black">
						<span>Password</span>
						<input
							class:input-error={!valid.password2}
							type="password"
							placeholder="Your Password"
							class="input input-bordered text-black"
							bind:value={loginData.password2}
						/>
					</label>
				</div>
				{#if submitLoading}
					<button
						type="submit"
						class="w-full bg-gradient-to-r from-mawoka-300 via-mawoka-200 to-mawoka-100 mt-4 py-2 rounded-2xl text-black font-semibold mb-2 flex justify-center"
						><Circle size="1.5" unit="rem" color="#000000" /></button
					>
				{:else}
					<button
						type="submit"
						class="block w-full bg-gradient-to-r from-mawoka-300 via-mawoka-200 to-mawoka-100 mt-4 py-2 rounded-2xl text-black font-semibold mb-2"
						disabled={!Object.values(valid).every((v) => v === true)}
						class:grayscale={!Object.values(valid).every((v) => v === true)}
						>Register</button
					>
				{/if}
				<span>
					<p class="text-red-600 ml-2">{feedback}</p>
				</span>
				<!-- <span
					><a href="/account/reset-password" class="text-sm ml-2 hover:text-blue-500 cursor-pointer"
						>Forgot Password?</a
					></span
				>
				<span
					><a href="/account/register" class="text-sm ml-2 hover:text-blue-500 cursor-pointer"
						>Don't have an account?</a
					></span
				> -->
			</form>
		</div>
	</div>
</div>

<div id="my-modal" class="modal" class:modal-open={modalOpen}>
	<div class="modal-box">
		<p class="text-black">{modalMessage}</p>
		<div class="modal-action">
			<button on:click={() => {modalOpen = false}} class="btn btn-primary">Close</button>
		</div>
	</div>
</div>
