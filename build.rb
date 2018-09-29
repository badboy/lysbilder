#!/usr/bin/env ruby
# encoding: utf-8

def add_doc_comment(content)
  content = content.split("\n")
  if content.first.empty?
    content.shift
  end

  content
    .map { |line| "/// #{line}" }
    .join("\n")
    .gsub(%r{^... $}, '///')
end

def make_slide_type(slide)
  if slide
    "Slide#{slide}"
  else
    "()"
  end
end

def slide_code(no, content, prev, nex)
  prev = make_slide_type(prev)
  nex = make_slide_type(nex)
  <<-EOF
#{add_doc_comment(content)}
pub struct Slide#{no};
impl Jump for Slide#{no} {
    type Previous = #{prev};
    type Next = #{nex};
}
  EOF
end

def jump_code
  <<-EOF
pub trait Jump {
    type Previous;
    type Next;
}
  EOF
end

file = ARGV.first or (puts "Need slide file" and exit 2)
data = IO.read(file)
slide_data = data.split("---\n").reject(&:empty?)
total_slides = slide_data.size
slides = slide_data.each_with_index.map { |slide,idx|
  prev = if idx==0 then nil else idx end
  nex  = if idx+1==total_slides then nil else idx+2 end
  [idx+1, slide, prev, nex]
}

puts jump_code
puts
slides.each do |data|
  puts slide_code(*data)
  puts
end
