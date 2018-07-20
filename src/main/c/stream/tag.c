#ifndef _HDR_HYPERBUILD_STREAM_TAG
#define _HDR_HYPERBUILD_STREAM_TAG

#include "../error/error.c"

#include "../rule/char/whitespace.c"
#include "../rule/tag/voidtags.c"

#include "../util/hbchar.h"
#include "../util/pipe.c"

#include "./helper/tagname.c"
#include "./helper/attr.c"
#include "./helper/script.c"

// Declare first before content.c, as content.c depends on it
void hbs_tag(hbu_pipe_t pipe);

#include "./content.c"

void hbs_tag(hbu_pipe_t pipe) {
  int self_closing = 0;

  hbu_pipe_require(pipe, '<');
  hbu_buffer_t opening_name = hbsh_tagname(pipe);
  while (1) {
    hbu_pipe_accept_while_predicate(pipe, &hbr_whitespace_check);

    if (hbu_pipe_accept_if(pipe, '>')) {
      break;
    }

    if (hbu_pipe_accept_if_matches(pipe, "/>")) {
      hbu_pipe_warn(pipe, "Self-closing tag");
      self_closing = 1;
      break;
    }

    // TODO Check for whitespace between attributes and before self-closing tag
    hbu_pipe_skip_while_predicate(pipe, &hbr_whitespace_check);

    hbsh_attr(pipe);
  }

  // Self-closing or void tag
  if (self_closing || hbr_voidtags_check(hbu_buffer_underlying(opening_name))) {
    return;
  }

  if (hbu_buffer_compare_lit(opening_name, "script") == 0) {
    // Script tag
    hbsh_script(pipe);
  } else {
    // Content
    hbs_content(pipe);
  }

  // Closing tag for non-void
  hbu_pipe_require(pipe, '<');
  hbu_pipe_require(pipe, '/');
  hbu_buffer_t closing_name = hbsh_tagname(pipe);
  hbu_pipe_require(pipe, '>');

  if (!hbu_buffer_equal(opening_name, closing_name)) {
    hbu_pipe_error(pipe, HBE_PARSE_UNCLOSED_TAG, "Tag not closed");
  }
}

#endif // _HDR_HYPERBUILD_STREAM_TAG
