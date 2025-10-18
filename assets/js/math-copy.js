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
      });
    });
    block.appendChild(btn);
  });

  // --- Inline math: click to copy, pointer cursor, no button ---
  document.querySelectorAll('.katex').forEach(function (block) {
    if (block.closest('.katex-display')) return; // skip if inside display math
    let annotation = block.querySelector('annotation[encoding="application/x-tex"]');
    if (!annotation) return;
    let latex = annotation.textContent;

    // Add class for pointer cursor
    block.classList.add('math-copy-inline');

    // Tooltip element (created once)
    let tooltip;
    function showTooltip(text) {
      if (!tooltip) {
        tooltip = document.createElement('span');
        tooltip.className = 'math-copy-tooltip';
        document.body.appendChild(tooltip);
      }
      tooltip.textContent = text;
      let rect = block.getBoundingClientRect();
      // Place below the math, centered
      tooltip.style.left = (window.scrollX + rect.left + rect.width / 2 - tooltip.offsetWidth / 2) + 'px';
      tooltip.style.top = (window.scrollY + rect.bottom + 8) + 'px';
      tooltip.classList.add('show');
      setTimeout(() => tooltip.classList.remove('show'), 1200);
    }

    block.addEventListener('mouseenter', function () {
      showTooltip('Copy LaTeX');
    });
    block.addEventListener('mouseleave', function () {
      if (tooltip) tooltip.classList.remove('show');
    });
    block.addEventListener('click', function (e) {
      navigator.clipboard.writeText(latex).then(function () {
        showTooltip('Copied!');
      });
    });
  });
});