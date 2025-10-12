require 'fileutils'
require 'digest'

module Jekyll
  class TikzConverter < Jekyll::Generator
    safe true
    priority :low

    def initialize(config = {})
      @used_hashes = []
    end

    def generate(site)
      @site = site
      tmp_dir = File.join(site.source, "_tikz_tmp")
      FileUtils.mkdir_p(tmp_dir)

      site.pages.each { |page| process_content(page, site.source) }
      site.posts.docs.each { |post| process_content(post, site.source) }

      site.config["tikz_used_hashes"] = @used_hashes
    end

    private

    def process_content(doc, source_dir)
      return unless doc.output_ext == ".html" || doc.extname =~ /md|markdown/

      content = doc.content
      new_content = content.gsub(/\$\$(.*?)\$\$/m) do
        inner = $1
        if inner =~ /(.*?)\\begin\{tikzpicture\}(.*?)\\end\{tikzpicture\}/m
          preamble = $1.strip
          tikz_body = $2.strip
          svg_path = generate_svg(tikz_body, source_dir, preamble)
          svg_url = File.exist?(svg_path) ? File.join('/assets/tikz', File.basename(svg_path)) : ""
          "<div class='tikz-svg'><img src='#{svg_url}' alt='TikZ SVG' /></div>"
        else
          "$$#{inner}$$"
        end
      end

      doc.content = new_content
    end

  def generate_svg(tikz_code, source_dir, preamble = "")
      # Compute a content-based hash to cache generated outputs and avoid rerendering identical TikZ.
      hash = Digest::MD5.hexdigest("#{preamble}\n#{tikz_code}")
      @used_hashes << hash

      tmp_dir = File.join(source_dir, "_tikz_tmp")
      svg_dir = File.join(source_dir, "assets", "tikz")
      FileUtils.mkdir_p(tmp_dir)
      FileUtils.mkdir_p(svg_dir)

      tex_file = File.join(tmp_dir, "#{hash}.tex")
      pdf_file = File.join(tmp_dir, "#{hash}.pdf")
      svg_file = File.join(svg_dir, "#{hash}.svg")

      # If SVG already exists, reuse it (caching).
      return svg_file if File.exist?(svg_file)

      tex_content = <<~TEX
        \\documentclass[tikz,border=2pt]{standalone}
        \\usepackage{tikz}
        \\usepackage{tikz-3dplot}
        \\usepackage{xcolor}
        \\begin{document}
        #{preamble}
        \\begin{tikzpicture}
        #{tikz_code}
        \\end{tikzpicture}
        \\end{document}
      TEX

      File.write(tex_file, tex_content)

      Dir.chdir(tmp_dir) do
        # Run pdflatex to produce a .pdf. Log an error and return empty string on failure.
        unless system("pdflatex", "-interaction=nonstopmode", "-halt-on-error", File.basename(tex_file))
          Jekyll.logger.error "TikZ Plugin:", "PDFLaTeX compile failed for TikZ block (#{hash})"
          return ""
        end

        # Convert PDF to SVG using pdf2svg.
        unless system("pdf2svg", File.basename(pdf_file), svg_file)
          Jekyll.logger.error "TikZ Plugin:", "pdf2svg failed for TikZ block (#{hash})"
          return ""
        end
      end

      svg_file
    end
  end
end

# === Cleanup temporary files and old SVGs ===
Jekyll::Hooks.register :site, :post_write do |site|
  tmp_dir = File.join(site.source, "_tikz_tmp")
  svg_dir = File.join(site.source, "assets", "tikz")
  used_hashes = site.config["tikz_used_hashes"] || []

  # Remove stale temporary files (tex, aux, log, pdf) that are not referenced by current site content.
  Dir.glob(File.join(tmp_dir, "*.{tex,aux,log,pdf}")).each do |path|
    hash = File.basename(path, File.extname(path))
    FileUtils.rm_f(path) unless used_hashes.include?(hash)
  end

  # Remove SVGs that are no longer used by any page/post (cleanup orphaned generated images).
  Dir.glob(File.join(svg_dir, "*.svg")).each do |path|
    hash = File.basename(path, ".svg")
    FileUtils.rm_f(path) unless used_hashes.include?(hash)
  end
end
