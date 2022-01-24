<script>
	import { Circle } from 'svelte-loading-spinners';
	import passwordChecker from 'zxcvbn';
	import tippy from 'sveltejs-tippy';
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/env';
	import Footer from "$lib/Footer.svelte";
	let loggedInSuccessfully = false;
	let loginData = {
		password1: '',
		email: ''
	};
	let valid = {
		password1: false,
		email: false
	};
	let hcaptchaSitekey = 'ee81b2a1-acf3-4d20-b2a4-a7ea94c7eba5';

	let hcaptcha = { execute: async (_a, _b) => ({ response: '' }), render: (_a, _b) => {} };
	let hcaptchaWidgetID;

	onMount(() => {
		if (browser) {
			hcaptcha = window.hcaptcha;
			if (hcaptcha.render) {
				hcaptchaWidgetID = hcaptcha.render('hcaptcha', {
					sitekey: hcaptchaSitekey,
					size: 'invisible',
					theme: 'dark'
				});
			}
		}
	});

	onDestroy(() => {
		if (browser) {
			hcaptcha = { execute: async () => ({ response: '' }), render: () => {} };
		}
	});

	let submitLoading = false;
	let modalMessage = '';
	let ErrorModalOpen = false;
	let SuccessModalOpen = false;
	let SuccessModalCode = '';

	const login = async () => {
		const { response } = await hcaptcha.execute(hcaptchaWidgetID, {
			async: true
		});
		submitLoading = true;
		const json_data = {
			email: loginData.email,
			password: loginData.password1
		};
		const res = await fetch(`/api/v1/users/login`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'h-captcha-response': response
			},
			body: JSON.stringify(json_data)
		});
		

		if (res.status === 200) {
			SuccessModalCode = await res.text();
			SuccessModalOpen = true;
		} else {
			let resp_text = await res.json();
			if (res.status === 400) {
				if (resp_text['detail'] === 'Invalid captcha') {
					modalMessage = 'The Captcha was invalid, please try again!';
					ErrorModalOpen = true;
				} else if (resp_text['detail'] === 'User not verified') {
					modalMessage = "You didn't confirm your email!";
					ErrorModalOpen = true;
				} else if (
					resp_text['detail'] === 'Wrong password' ||
					resp_text['detail'] === 'User not found'
				) {
					modalMessage = 'Email and/or password is/are wrong!';
					ErrorModalOpen = true;
				}
			} else {
				modalMessage = 'The good old unexpected error occured! I hope you like it!';
				ErrorModalOpen = true;
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
	$: valid.password1 = loginData.password1.length >= 4;
	let emailRegex = /^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6}$/;
	$: valid.email = emailRegex.exec(loginData.email) !== null;
	let passwdstrength;
</script>

<svelte:head>
	<script src="https://js.hcaptcha.com/1/api.js?render=explicit" async defer></script>
</svelte:head>

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
					<div
						id="hcaptcha"
						class="h-captcha"
						data-sitekey={hcaptchaSitekey}
						data-size="normal"
						data-theme="dark"
					/>
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
						>Log In</button
					>
				{/if}
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
<Footer />

<div class="modal" class:modal-open={ErrorModalOpen}>
	<div class="modal-box">
		<p class="text-black">{modalMessage}</p>
		<div class="modal-action">
			<button
				on:click={() => {
					ErrorModalOpen = false;
				}}
				class="btn btn-primary">Close</button
			>
		</div>
	</div>
</div>

<div class="modal" class:modal-open={SuccessModalOpen}>
	<div class="modal-box">
		<p class="text-black text-center">Here is your code. This code will last just 5mins.</p>
		<p class="text-center"><code class="text-black text-xl text-center select-all">{SuccessModalCode}</code></p>
		
		<div class="modal-action">
			<button
				on:click={() => {
					SuccessModalOpen = false;
				}}
				class="btn btn-primary">Close</button
			>
		</div>
	</div>
</div>
