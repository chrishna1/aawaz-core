<script>
  import { onMount, setContext } from 'svelte'
  import { writable } from 'svelte/store';
  import axios from 'redaxios'
  import { t } from './i18n'
  import Comment from './lib/Comment.svelte'
  import Reply from './lib/CommentForm.svelte'
  import Brand from './lib/Brand.svelte'
  import LoginStatus from './lib/LoginStatus.svelte';

  export let attrs

  let isLoggedIn = false;
  let userInfo;
  let commentsResult
  let error
  let loading = true
  let message = ''
  let theme = attrs.theme || 'light'
  console.log('theme :>> ', theme);


  const api = axios.create({
    baseURL: attrs.host,
    withCredentials: true // adds cookie..
  })


  setContext('api', api)
  setContext('attrs', attrs)
  setContext('refresh', getComments)
  setContext('setMessage', setMessage)

  let reactiveStates = {
    isLoggedIn,
    userInfo,
  }

  let state = writable(reactiveStates);
  setContext('state', state);

  function setMessage(msg) {
    message = msg
  }

  async function getComments() {
    loading = true
    try {
      const res = await api.get(`/api/v1/comments`, {
        params: {
          url: window.parent.location.href,
        //   appId: attrs.appId,
        //   pageId: attrs.pageId,
        },
      })
      commentsResult = res.data
    } catch (e) {
      error = e
    } finally {
      loading = false
    }
  }

  async function fetchUserInfo() {
      try {
          const res = await api.get(`/api/v1/me`, {
            params: {
                url: window.parent.location.href,
            //   appId: attrs.appId,
            //   pageId: attrs.pageId,
            },
          })
          if(!res.data) {
            return
          }
          isLoggedIn = true;
          userInfo = res.data;

          $state = {
            isLoggedIn,
            userInfo
          }

      } catch (e) {

        $state = {
          isLoggedIn: false,
          userInfo: null
        }
        console.error(e)
        // error = e;
      }
  }


  onMount(() => {

    function onMessage(e) {
      try {
        const msg = JSON.parse(e.data)
        if (msg.from === 'cusdis') {
          switch (msg.event) {
            case 'setTheme':
              {
                theme = msg.data
              }
              break
          }
        }
      } catch (e) {}
    }
    window.addEventListener('message', onMessage)

    return () => {
      window.removeEventListener('message', onMessage)
    }
  })

  onMount(() => {
    getComments();
    fetchUserInfo();
  })

</script>

{#if !error}
  <div class:dark={theme === 'dark'}>

    {#if isLoggedIn}
      <div class="text-end">
        <LoginStatus />
      </div>
    {/if}

    {#if message}
      <div class="p-2 mb-4 bg-blue-500 text-white">
        {message}
      </div>
    {/if}

    <Reply />

    <div class="my-8" />

    <div class="mt-4">
      {#if loading}
        <div class="text-gray-900 dark:text-gray-100">
          {t('loading')}...
        </div>
      {:else}
        {#each commentsResult as comment (comment.id)}
          <Comment {comment} />
        {/each}
      {/if}
    </div>

    <div class="my-8" />

    <Brand />
  </div>

{:else}

<div class="text-center">
    <p style="color: red;">{error}</p>
</div>

{/if}
