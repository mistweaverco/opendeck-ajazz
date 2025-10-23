import { redirect } from '@sveltejs/kit';

export function load() {
	redirect(302, 'https://github.com/mistweaverco/opendeck-ajazz/releases/latest');
}
