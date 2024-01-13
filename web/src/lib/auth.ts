import { writable } from 'svelte/store';
import type { User } from '../types/user';

export const user = writable<User | null>();
