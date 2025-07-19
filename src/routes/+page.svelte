<script lang="ts">
    import { Button } from "$lib/components/ui/button/index.js";
    import { Skull } from "@lucide/svelte";
</script>

<div
    class="min-h-screen flex items-center justify-center p-4"
    style="background: linear-gradient(135deg, #acdff0 0%, #ffeec9 50%, #ffdbce 100%);"
>
    <div
        class="max-w-lg w-full bg-white/90 backdrop-blur-sm rounded-lg border border-white/20 p-8 text-center shadow-lg"
    >
        <div class="mb-8">
            <h1 class="text-3xl font-semibo ld text-gray-900 mb-3">
                Welcome to SvelteKit with Rolldown × Tauri with Servo rendering
                × Biome × Tailwind × ShadCn × Bun
            </h1>
            <p class="text-gray-700">
                A powerful desktop app framework combining Rust and modern web
                tech
            </p>
        </div>

        <div class="space-y-6">
            <div class="flex justify-center flex-wrap gap-2">
                <span
                    class="bg-orange-100 text-orange-800 px-3 py-1 rounded-md text-sm font-medium"
                >
                    Svelte v5 + SvelteKit v2
                </span>
                <span
                    class="bg-blue-100 text-blue-800 px-3 py-1 rounded-md text-sm font-medium"
                >
                    Vite v7 (with Rolldown)
                </span>
                <span
                    class="bg-sky-100 text-sky-800 px-3 py-1 rounded-md text-sm font-medium"
                >
                    Tauri v2 (with Servo rendering engine)
                </span>
                <span
                    class="bg-green-100 text-green-800 px-3 py-1 rounded-md text-sm font-medium"
                >
                    Biome v2
                </span>
                <span
                    class="bg-purple-100 text-purple-800 px-3 py-1 rounded-md text-sm font-medium"
                >
                    Tailwind v4
                </span>
                <span
                    class="bg-purple-100 text-purple-800 px-3 py-1 rounded-md text-sm font-medium"
                >
                    Lucide Icons
                </span>
                <span
                    class="bg-yellow-100 text-yellow-800 px-3 py-1 rounded-md text-sm font-medium"
                >
                    ShadCn
                </span>
                <span
                    class="bg-red-100 text-red-800 px-3 py-1 rounded-md text-sm font-medium"
                >
                    Bun v1.2
                </span>
                <Button variant="primary">ShadCn Button</Button>
            </div>
        </div>
    </div>
</div>
