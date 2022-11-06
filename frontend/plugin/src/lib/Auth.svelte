<script>
  import { Button, Modal } from 'flowbite-svelte'
  export let defaultModal = false;
  let placement = 'top-center';
  let comment_fragment = '#aawaz'
  let github_signup = `http://localhost:8080/auth/github?url=${window.parent.location.href}${comment_fragment}`
</script>

<Modal title="Sign up" autoclose={false} bind:open={defaultModal} {placement} backdropClasses={'bg-gray-500 bg-opacity-50 dark:bg-opacity-80'}>

  <a href={github_signup}>Sign up with Github</a>
</Modal>
