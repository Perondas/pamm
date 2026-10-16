type Config = {
  title: string;
  description: string;
  lang: string;
  favicon: string;
  repoUrl: string;
  og: {
    image: string;
    imageAlt: string;
    imageType: string;
    imageWidth: string;
    imageHeight: string;
  };
};

export const siteConfig: Config = {
  // Appears in the nav logo, browser title bar, and OG site name.
  title: "pamm",
  // Used as the default meta description and OG description on pages without their own.
  description:
    "pamm is a lightweight, fast Arma 3 mod manager for communities that host their own mods.",
  // BCP 47 language tag for the HTML lang attribute (e.g. "en", "de", "fr", "zh-TW").
  lang: "en",
  favicon: "/logo.png",
  repoUrl: "https://github.com/Perondas/pamm",
  og: {
    // Path is relative to /public.
    image: "/logo.png",
    imageAlt: "The pamm logo",
    imageType: "image/png",
    imageWidth: "1000",
    imageHeight: "1000",
  },
};
