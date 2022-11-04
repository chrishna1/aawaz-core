<script>
  import { Button, Modal } from 'flowbite-svelte'
  export let defaultModal = false;
  let placement = 'top-center';
</script>

<Modal title="Sign up" autoclose={false} bind:open={defaultModal} {placement} backdropClasses={'bg-gray-500 bg-opacity-50 dark:bg-opacity-80'}>

  <a href="http://localhost:8080/auth/github">Sign up with Github</a>

</Modal>
