<script>
  import { Button, Dropdown, DropdownItem, DropdownDivider, DropdownHeader, Chevron } from 'flowbite-svelte'
  import { getContext } from 'svelte'

  let username;
  const api = getContext('api')
  let state = getContext('state')

  async function logout() {
    await api.get('/logout')
    $state = {
      isLoggedIn: false,
      userInfo: null
    }
  }
</script>


{#if $state.isLoggedIn}
  <Button><Chevron>{$state.userInfo.username}</Chevron></Button>
  <Dropdown>
    <div slot="header" class="px-4 py-2">
      <span class="block text-sm text-gray-900 dark:text-white"> {$state.userInfo.name} </span>
      <span class="block truncate text-sm font-medium"> {$state.userInfo.email} </span>
    </div>
    <DropdownItem>Settings</DropdownItem>
    <DropdownItem slot="footer" on:click={logout}>Log out</DropdownItem>
  </Dropdown>
{/if}
