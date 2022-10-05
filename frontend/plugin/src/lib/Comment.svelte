<script>
  import { getContext } from 'svelte'

  import { t } from '../i18n'
  import Reply from './CommentForm.svelte'

  export let comment
  export let showReplyForm = false
  export let isChild = false

  const { showIndicator } = getContext('attrs')

</script>

<div
  class="my-4"
  class:pl-4={isChild}
  class:border-l-2={isChild}
  class:border-color-gray-200={isChild}
  class:cusdis-indicator={showIndicator}
>
  <div class="flex items-center">
    <div class="mr-2 font-medium dark:text-gray-100">
      {comment.user.name}
    </div>

    {#if comment.moderatorId}
      <div class="mr-2 dark:bg-gray-500 bg-gray-200 text-xs py-0.5 px-1 rounded dark:text-gray-100">
        <span>{t('mod_badge')}</span>
      </div>
    {/if}
  </div>

  <div class="text-gray-500 text-sm dark:text-gray-400" title="{comment.created_at}">
    {new Date(comment.created_at).toLocaleDateString('en-us', {
        year:"numeric",
        month:"short",
        day:"numeric",
        hour: "numeric",
        minute: "numeric"
    })}

  </div>

  <div class="text-gray-500 my-2 dark:text-gray-200">
    {comment.content}
  </div>

  <!-- {#if comment.replies.data.length > 0}
    {#each comment.replies.data as child (child.id)}
      <svelte:self isChild={true} comment={child} />
    {/each}
  {/if} -->

  <div>
    <button
      class="font-medium text-sm text-gray-500 dark:bg-transparent dark:text-gray-100"
      type="button"
      on:click={(_) => {
        showReplyForm = !showReplyForm
      }}>{t('reply_btn')}</button
    >
  </div>


  {#if showReplyForm}
    <div class="mt-4 pl-4 border-l-2 border-gray-200">
      <Reply
        parentId={comment.id}
        onSuccess={() => {
          showReplyForm = false
        }}
      />
    </div>
  {/if}


</div>
