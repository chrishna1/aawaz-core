<script>
  import { Button, Modal } from 'flowbite-svelte'
  export let defaultModal = false;
  let placement = 'top-center';
  let github_signup = `http://localhost:8080/auth/github?url=${window.parent.location.href}`
  let google_signup = `http://localhost:8080/auth/google?url=${window.parent.location.href}`
</script>

<Modal title="Sign up" autoclose={false} bind:open={defaultModal} {placement} backdropClasses={'bg-gray-500 bg-opacity-50 dark:bg-opacity-80'}>
  <Button><a href={github_signup}>Sign up with Github</a></Button>
  <Button><a href={google_signup}>Sign up with Google</a></Button>
</Modal>
