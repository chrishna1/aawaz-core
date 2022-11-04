<script>
  import { getContext } from 'svelte'
  import { t } from '../i18n'
  export let parentId

  import Auth from './Auth.svelte'

  // form data
  let content = ''
  let nickname = ''
  let email = ''

  let loading = false
  let authPopup = false

  export let onSuccess

  const api = getContext('api')
  const setMessage = getContext('setMessage')
  const { appId, pageId, pageUrl, pageTitle } = getContext('attrs')
  const refresh = getContext('refresh')
  let { getIsLoggedIn } = getContext('isLoggedIn')
  const isLoggedIn = getIsLoggedIn()
  console.log("xxx isLoggedIn::", isLoggedIn)



  async function addComment() {
    if (!isLoggedIn) {
      authPopup = true
      return
    }

    if (!content) {
      alert(t('content_is_required'))
      return
    }

    if (!nickname) {
      alert(t('nickname_is_required'))
      return
    }

    try {
      loading = true
      const res = await api.post('/api/v1/comment', {
        // appId,
        // pageId,
        // content,
        // nickname,
        // email,
        // parentId,
        // pageUrl,
        // pageTitle,
        "url": (window.location != window.parent.location)
            ? window.parent.location.href
            : document.location.href,
        "user_id": 1251,
        "parent_id": parentId,
        "content": content
      })
      await refresh()
      teardown()
      setMessage(t('comment_has_been_sent'))
    } finally {
      loading = false
    }
  }

  function teardown() {
    content = ''
    nickname = ''
    email = ''
    onSuccess && onSuccess()
  }

</script>

<div class="grid grid-cols-1 gap-4">

  {#if authPopup}
    <Auth defaultModal={authPopup}/>
  {/if}

  <div class="grid grid-cols-2 gap-4">
    <div class="px-1">
      <label class="mb-2 block dark:text-gray-200" for="nickname">{t('nickname')}</label>
      <input
        name="nickname"
        class="w-full p-2 border border-gray-200 bg-transparent dark:text-gray-100 dark:outline-none"
        type="text"
        bind:value={nickname}
      />
    </div>
    <div class="px-1">
      <label class="mb-2 block dark:text-gray-200" for="email">{t('email')}</label>
      <input
        name="email"
        class="w-full p-2 border border-gray-200 bg-transparent  dark:text-gray-100 dark:outline-none"
        type="email"
        bind:value={email}
      />
    </div>
  </div>

  <div class="px-1">
    <label class="mb-2 block dark:text-gray-200" for="reply_content">{t('reply_placeholder')}</label>
    <textarea
      name="reply_content"
      class="w-full p-2 border border-gray-200 h-24 bg-transparent dark:text-gray-100 dark:outline-none"
      bind:value={content}
    />
  </div>

  <div class="px-1">
    <button

      class="text-sm bg-gray-200 p-2 px-4 font-bold"
      class:cusdis-disabled={loading}
      on:click={addComment}>{loading ? t('sending') : t('post_comment')}</button
    >
  </div>
</div>
