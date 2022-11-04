<script>
  import { Button, Dropdown, DropdownItem, DropdownDivider, DropdownHeader, Chevron } from 'flowbite-svelte'
  import { getContext } from 'svelte'

  let username;
  const api = getContext('api')
  let { getUserInfo } = getContext('userInfo')
  let { getIsLoggedIn } = getContext('isLoggedIn')
  const userInfo = getUserInfo()
  const isLoggedIn = getIsLoggedIn()
  console.log("isLoggedIn::", isLoggedIn)
  console.log("userInfo::", userInfo)

  async function logout() {
    await api.get('/logout')
    isLoggedIn = false
  }
</script>


{#if isLoggedIn}
  <Button><Chevron>{userInfo.username}</Chevron></Button>
  <Dropdown>
    <div slot="header" class="px-4 py-2">
      <span class="block text-sm text-gray-900 dark:text-white"> {userInfo.name} </span>
      <span class="block truncate text-sm font-medium"> {userInfo.email} </span>
    </div>
    <DropdownItem>Settings</DropdownItem>
    <DropdownItem slot="footer" on:click={logout}>Log out</DropdownItem>
  </Dropdown>
{/if}
