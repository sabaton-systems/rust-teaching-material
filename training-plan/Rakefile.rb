TARGET_DIR='target'
autoload :FileUtils, 'fileutils'
require 'asciidoctor'
#require 'asciidoctor-revealjs'
require 'asciidoctor-pdf'

# TODO: add ability to build only a single assignment/presentation:
# https://stackoverflow.com/questions/9539324


desc 'Build training plan'
task :trainingplan  do
  # FIXME: this will break if there's a name collision
  #FileUtils.cp Dir.glob('./default/*/*.{svg,jpg}'), './target'
  #FileUtils.cp './presentations/slides.css', './target/slides.css'

  (FileList.new './default/*.adoc').each do |doc|
    puts "Converting #{doc}"
    name = File.basename (File.dirname doc)
    Asciidoctor.convert_file doc,
      safe: :unsafe,
      attributes: <<-ATTRS,
        customcss=slides.css
        revealjsdir=https://cdn.jsdelivr.net/npm/reveal.js@4.3.1
        highlightjs-theme=https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.15.10/styles/idea.min.css
        icons=font
        revealjs_height=1080
        revealjs_history=true
        revealjs_theme=simple
        revealjs_transition=none
        revealjs_width=1920
        source-highlighter=highlightjs
        docinfo=shared
        docinfodir=../
      ATTRS
      backend: 'pdf',
      to_file: "#{TARGET_DIR}/#{name}.pdf",
      mkdirs: true
  end
end

#desc 'Build index'
#task :index do
#  Asciidoctor.convert_file './presentations/index.adoc',
#    safe: :unsafe,
#    to_file: "#{TARGET_DIR}/index.html",
#    mkdirs: true
#end

desc 'Build all'
task default: [:trainingplan]

desc 'Clean the build directory'
task :clean do
  FileUtils.remove_entry_secure TARGET_DIR if File.exist? TARGET_DIR
end
