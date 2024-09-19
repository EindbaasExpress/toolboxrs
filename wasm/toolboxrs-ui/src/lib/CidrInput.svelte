<script lang="ts">
    import {validator, formatHelper} from "../utils/cidr"
  
  export let inputValue = '';
  let userInput = '';
  let inputWidth: any;
  let hiddenText: any;
  export let placeholder = "X.X.X.X/Y";
  let greyedOutGuideText = "";
  let indicationColor = "grey";
  // Function to update the width of the input field based on the text width
  function updateWidth() {
    if (hiddenText) {
      hiddenText.textContent = userInput || placeholder; // Prevents zero width
      inputWidth = hiddenText.offsetWidth + 'px';
    }
  }

  // Update width whenever the user input changes
  $: userInput, updateWidth();
  
    const inputHandler = (event: Event & { target: any; }) => {
    const {input, guide} = formatHelper(event.target?.value);
    const noMatch = guide == undefined
    greyedOutGuideText = noMatch ? greyedOutGuideText : guide;
    
    ({indicationColor, inputValue} = validator(input, noMatch));

    }

</script>

<style>
  .input-container {
    position: relative;
  }
  .input-field {
    width: auto;
    min-width: 1ch; /* Ensure there's a minimum width */
    box-sizing: content-box;
    padding: 5px;
    outline: 2px solid red; /* Set the outline color to red */
  }
  .hidden-text {
    position: absolute;
    visibility: hidden;
    white-space: pre;
    font: inherit; /* Ensure the same font style is used */
  }
  .dynamic-text {
    left: calc(100% - 100px); /* Align text next to input field */
    white-space: nowrap; /* Prevent text from wrapping */
  }
</style>

<div class="input-container">
    <input 
    class="input-field"
    type="text" 
    bind:value={userInput} 
    {placeholder}
    on:input={inputHandler}
    style="width: {inputWidth}; outline: 2px solid {indicationColor}"
/>
  <!-- Hidden element used to measure text width -->
  <span class="hidden-text" bind:this={hiddenText}>X.X.X.X/Y</span>
  <span class="dynamic-text">{greyedOutGuideText}</span>
</div>
