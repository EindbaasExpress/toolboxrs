<script lang="ts">
    import type { ActionInput } from "./types";
    import { reducer } from "./transformations";
    export let radioValue: string | undefined = "";
    export let subOptionValue: string | undefined = "";
    export let inputValue = "";
    
    
    const actionInput:ActionInput = {
      action: {
          type: radioValue,
          subOption: subOptionValue,
      },
      inputValue,
    }  
  </script>
  
  <div class="container">
    <p>
      {@html reducer(actionInput)} 
    </p>
  </div>  

  <style>
  .container {
    width: 100%;
    background-color: #796AF3;
    padding: 1.5em;
    filter: drop-shadow(0 0 2em #796AF3);

  }
  p {
    font-size: calc(0.7vw + 1vh);
    white-space: pre-wrap;
  }
  </style>