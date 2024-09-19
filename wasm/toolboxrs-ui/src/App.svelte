<script lang="ts">
  import toolboxrsLogo from './assets/toolboxrslogo.svg'
  import Radio from './lib/Radio.svelte'
  import OutputField from './lib/OutputField.svelte';
  import CidrInput from './lib/CidrInput.svelte';
  import type { OptionsSet } from "./lib/types";

  // wasmModule.greet("yes")
  
  // cidr info based on ip-address
  //base64encode/decode/url-safe/unsafe
  // Hash based on specific algorithm
  // 

  import {writable, type Writable} from 'svelte/store'

  let inputValue = "";
	let radioValue: Writable<string | undefined> = writable();
  let radioAvailableSubOptions: Writable<{ value: string; label: string; }[] | undefined> = writable();
  let subOptionValue: Writable<string | undefined> = writable();
	
	const options:OptionsSet[] = [{
		value: 'cidr',
		label: 'Get CIDR info for ip-address',
    suboptions: [],
	}, {
		value: 'base64',
		label: 'Perform base64-related en/decoding',
    suboptions: [
      {value: "encode", label: "encode"},  
      {value: "decode", label: "decode"},
    ],
	}, {
		value: 'hash',
		label: 'Hash a string value',
    suboptions: [
      {value:"Sha256", label: "SHA256"},
      {value:"Sha384", label: "SHA384"},
      {value:"Sha512", label: "SHA512"},
      {value:"Sha3_256", label: "SHA3 256"},
      {value:"Sha3_384", label: "SHA3 384"},
      {value:"Sha3_512", label: "SHA3 512"},
      {value:"Blake3", label: "Blake3"},
    ],
	}]



  radioValue.subscribe((radioValue:string|undefined) => {
    $subOptionValue = undefined
    radioAvailableSubOptions.set(options.find(e => e.value == $radioValue)?.suboptions)
  });

  
</script>

<main>
  <div id="toolboxrsapp">
  <div>
    <img src={toolboxrsLogo} class="logo" alt="Toolboxrs Logo" />
  </div>
  <h1>toolboxrs</h1>
  <div class="options">
    <Radio {options} legend='Select Action to Perform' bind:userSelected={$radioValue}/>
    {#if $radioValue && ($radioAvailableSubOptions && $radioAvailableSubOptions.length > 0)}
      {#key $radioValue}
        <Radio options={
          $radioAvailableSubOptions
        } legend='Select the type of Action' bind:userSelected={$subOptionValue}/>
      {/key}
    {/if}
    {#if $radioValue == "cidr"}
    <CidrInput bind:inputValue={inputValue} placeholder="X.X.X.X/Y" />
    {:else}
    <input bind:value={inputValue} placeholder="enter value" />
    {/if}
    
  
    <p class="guide">
      when {$radioValue} {$subOptionValue ? `and ${$subOptionValue}` : ""} is selected, your output is:
    </p>
    {#if inputValue}
    {#key inputValue + $radioValue + $subOptionValue }
    <OutputField radioValue={$radioValue} subOptionValue={$subOptionValue} bind:inputValue={inputValue}/>
    {/key}
    {:else}
    <div class="container">
    </div>  
    {/if}   
  </div>

  <p class="footer">
    Check out the <a href="https://github.com/EindbaasExpress/toolboxrs" target="_blank" rel="noreferrer">Github Project</a> for this tool. Powered by Rust, WebAssembly, Svelte and Vite
  </p>
  </div>
</main>

<style>
  .logo {
    height: 15em;
    padding: 0.1em;
    will-change: filter;
    transition: filter 300ms;
  }
  .container {
    width: 100%;
    background-color: #796AF3;
    padding: 1.5em;
    filter: drop-shadow(0 0 1em #796AF3);
  }
  .footer {
    padding: 0.5em;
  }
  #toolboxrsapp {
    height: 100%;
    width: 100%;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #796AF3);
  }
  input {
    width: 500px;
    height: 50px;
  }
  .options {
    font-size: calc(0.7vw + 1vh);
  }
</style>
