function getThemeCookie() {
  var themeCookie = document.cookie
    .split(";")
    .find((c) => c.trim().startsWith("theme="));
  if (themeCookie) {
    return themeCookie.split("=")[1].trim();
  }
  return null;
}

// Set initial theme from cookie immediately
(function () {
  var savedTheme = getThemeCookie();
  if (savedTheme) {
    if (savedTheme === "light") {
      document.documentElement.classList.remove("dark-theme");
      document.documentElement.classList.add("light-theme");
    } else {
      document.documentElement.classList.remove("light-theme");
      document.documentElement.classList.add("dark-theme");
    }
  }
})();

function createThemeSwitcher() {
  var headerLinks = document.getElementById("header-links");
  if (!headerLinks) {
    console.log("Header links not found");
    return;
  }
  var li = document.createElement("li");
  var a = document.createElement("a");
  var img = document.createElement("img");
  updateSwitcherIcon(img, isLightMode());
  a.href = "#";

  a.addEventListener("click", function (e) {
    e.preventDefault();
    var isLight = isLightMode();
    if (isLight) {
      document.documentElement.classList.remove("light-theme");
      document.documentElement.classList.add("dark-theme");
      document.cookie = "theme=dark;path=/;max-age=31536000";
    } else {
      document.documentElement.classList.remove("dark-theme");
      document.documentElement.classList.add("light-theme");
      document.cookie = "theme=light;path=/;max-age=31536000";
    }
    updateSwitcherIcon(img, !isLight);
  });

  a.appendChild(img);
  li.appendChild(a);
  headerLinks.append(li);

  // Add media query listener
  var colorSchemeQuery = window.matchMedia("(prefers-color-scheme: light)");
  colorSchemeQuery.addEventListener("change", function (e) {
    // Only update if no theme cookie is set
    if (!getThemeCookie()) {
      updateSwitcherIcon(img, e.matches);
    }
  });

  function isLightMode() {
    var themeCookie = getThemeCookie();
    if (themeCookie) {
      return themeCookie === "light";
    }
    return (
      window.matchMedia("(prefers-color-scheme: light)").matches ||
      document.documentElement.classList.contains("light-theme")
    );
  }

  function updateSwitcherIcon(img, isLightMode) {
    img.src = isLightMode ? "LIGHT_MODE_ICON" : "DARK_MODE_ICON";
    img.alt = isLightMode ? "Light Mode" : "Dark Mode";
  }
}

function initScrollSpy() {
  const tocSticky = document.getElementById("toc-sticky");
  const tocInline = document.getElementById("toc-inline");
  if (!tocSticky && !tocInline) return;

  // Include both headings and bare anchors
  const elements = document.querySelectorAll(
    ".post-body :is(h3[id], h4[id], h5[id], h6[id]), .post-body a[id]"
  );
  const tocStickyLinks = tocSticky ? tocSticky.querySelectorAll("a") : [];
  const tocInlineLinks = tocInline ? tocInline.querySelectorAll("a") : [];

  function updateActiveLink() {
    // Get current scroll position, accounting for some offset
    const scrollPos = window.scrollY + 100;

    // Find the element that's currently in view
    let currentElement = null;
    elements.forEach((element) => {
      if (element.offsetTop <= scrollPos) {
        currentElement = element;
      }
    });

    // Remove active class from all links
    tocStickyLinks.forEach((link) => {
      link.classList.remove("active");
    });
    tocInlineLinks.forEach((link) => {
      link.classList.remove("active");
    });

    // Add active class to corresponding links
    if (currentElement) {
      if (tocSticky) {
        const stickyLink = tocSticky.querySelector(
          `a[href="#${currentElement.id}"]`
        );
        if (stickyLink) {
          stickyLink.classList.add("active");
        }
      }
      if (tocInline) {
        const inlineLink = tocInline.querySelector(
          `a[href="#${currentElement.id}"]`
        );
        if (inlineLink) {
          inlineLink.classList.add("active");
        }
      }
    } else {
      if (tocStickyLinks.length) tocStickyLinks[0].classList.add("active");
      if (tocInlineLinks.length) tocInlineLinks[0].classList.add("active");
    }
  }

  // Update on scroll
  window.addEventListener("scroll", updateActiveLink);

  // Initial update
  updateActiveLink();
}

document.addEventListener("DOMContentLoaded", function () {
  createThemeSwitcher();
  initScrollSpy();
});
