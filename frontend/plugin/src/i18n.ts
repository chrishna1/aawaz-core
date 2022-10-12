import en from "./lang/en";

export function t(key: string) {
    const LOCALE_KEY: string = "AAWAZ_DICTIONARY";
    const locale = window[LOCALE_KEY] || en;
    const content = locale[key] || en[key]; // use english as fallback
    return content;
}
