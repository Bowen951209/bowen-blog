document.addEventListener('DOMContentLoaded', function () {
  // --- Display math: button on hover ---
  document.querySelectorAll('.katex-display').forEach(function (block) {
    let katexSpan = block.querySelector('.katex');
    if (!katexSpan) return;
    let annotation = katexSpan.querySelector('annotation[encoding="application/x-tex"]');
    if (!annotation) return;
    let latex = annotation.textContent;

    let btn = document.createElement('button');
    btn.className = 'math-copy-btn';
    btn.type = 'button';
    btn.innerHTML = '<i class="fa-regular fa-copy"></i>';
    btn.title = 'Copy LaTeX';

    btn.addEventListener('click', function (e) {
      e.stopPropagation();
      navigator.clipboard.writeText(latex).then(function () {
        btn.innerHTML = '<i class="fa-solid fa-check"></i>';
        setTimeout(() => {
          btn.innerHTML = '<i class="fa-regular fa-copy"></i>';
        }, 1200);
      }).catch(function () {
        btn.innerHTML = '<i class="fa-solid fa-xmark"></i>'; // show error
        setTimeout(() => {
          btn.innerHTML = '<i class="fa-regular fa-copy"></i>';
        }, 1200);
      });
    });
    block.appendChild(btn);
  });

  // --- Inline math: click to copy, pointer cursor, no button, shared tooltip, keyboard accessible ---
  let sharedTooltip = null;
  let tooltipTimeout = null;

  function showTooltipForBlock(block, text) {
    if (!sharedTooltip) {
      sharedTooltip = document.createElement('span');
      sharedTooltip.className = 'math-copy-tooltip';
      document.body.appendChild(sharedTooltip);
    }
    sharedTooltip.textContent = text;
    sharedTooltip.classList.add('show');
    // Wait for DOM update before measuring
    requestAnimationFrame(() => {
      let rect = block.getBoundingClientRect();
      sharedTooltip.style.left = (window.scrollX + rect.left + rect.width / 2 - sharedTooltip.offsetWidth / 2) + 'px';
      sharedTooltip.style.top = (window.scrollY + rect.bottom + 8) + 'px';
    });
    if (tooltipTimeout) clearTimeout(tooltipTimeout);
    tooltipTimeout = setTimeout(() => {
      if (sharedTooltip) sharedTooltip.classList.remove('show');
    }, 1200);
  }

  function hideTooltip() {
    if (sharedTooltip) sharedTooltip.classList.remove('show');
    if (tooltipTimeout) clearTimeout(tooltipTimeout);
  }

  document.querySelectorAll('.katex').forEach(function (block) {
    if (block.closest('.katex-display')) return; // skip if inside display math
    let annotation = block.querySelector('annotation[encoding="application/x-tex"]');
    if (!annotation) return;
    let latex = annotation.textContent;

    block.classList.add('math-copy-inline');
    block.setAttribute('tabindex', '0');
    block.setAttribute('role', 'button');
    block.setAttribute('aria-label', 'Copy LaTeX');

    block.addEventListener('mouseenter', function () {
      showTooltipForBlock(block, 'Copy LaTeX');
    });
    block.addEventListener('mouseleave', hideTooltip);
    block.addEventListener('focus', function () {
      showTooltipForBlock(block, 'Copy LaTeX');
    });
    block.addEventListener('blur', hideTooltip);

    function doCopy() {
      navigator.clipboard.writeText(latex).then(function () {
        showTooltipForBlock(block, 'Copied!');
      }).catch(function () {
        showTooltipForBlock(block, 'Copy failed');
      });
    }

    block.addEventListener('click', function (e) {
      doCopy();
    });
    block.addEventListener('keydown', function (e) {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        doCopy();
      }
    });
  });
});