<!--
Copyright: Ankitects Pty Ltd and contributors
License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
-->
<script lang="ts">
    let className: string = "";
    export { className as class };

    export let itemsCount: number = 0;
    export let itemHeight: number;
    export let bottomOffset: number = 0;

    let container: HTMLElement;
    let scrollTop: number = 0;

    $: containerHeight = container
        ? Math.floor(
              (document.documentElement.clientHeight -
                  container.offsetTop -
                  bottomOffset) /
                  itemHeight,
          ) * itemHeight
        : 0;
    $: innerHeight = Math.max(containerHeight, itemsCount * itemHeight);
    $: sliceLength = Math.ceil(containerHeight / itemHeight);
    $: startIndex = Math.floor(scrollTop / itemHeight);
    $: endIndex = Math.min(startIndex + sliceLength, itemsCount);
    $: slice = new Array(endIndex - startIndex).fill(0).map((_, i) => startIndex + i);

    window.addEventListener("resize", () => {
        containerHeight = containerHeight;
    });
</script>

<div
    class="outer"
    style="--container-height: {containerHeight + 1}px"
    bind:this={container}
    on:scroll={() => (scrollTop = container.scrollTop)}
>
    <div class="inner" style="height: {innerHeight}px;">
        <table
            class="table {className}"
            tabindex="-1"
            style="margin-top: {scrollTop}px"
        >
            <slot name="headers" />
            {#each slice as index (index)}
                <slot name="row" {index} />
            {/each}
        </table>
    </div>
</div>

<style lang="scss">
    .outer {
        width: 100%;
        overflow: auto;

        height: var(--container-height);
        margin: 0 auto;
    }

    .inner {
        position: relative;
        overflow-y: visible;

        width: 100%;
    }

    .table {
        // Prevent infinite scrolling
        position: absolute;

        border-collapse: collapse;
        white-space: nowrap;

        :global(th),
        :global(td) {
            text-overflow: ellipsis;
            overflow: hidden;
            border: 1px solid var(--border-subtle);
            padding: 0.25rem 0.5rem;
            max-width: 15em;
        }

        :global(th) {
            background: var(--border);
            text-align: center;
        }
    }
</style>
