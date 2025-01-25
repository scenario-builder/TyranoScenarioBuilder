var sidebarBtn = document.querySelector('.sidebar-toggle');
var sidebar = document.querySelector('.sidebar');

if(sidebarBtn && sidebar) {
  sidebarBtn.addEventListener('click', function() {
    sidebarBtn.classList.toggle('sidebar-closed');
    sidebar.classList.toggle('sidebar-closed');
  })
}

const a_block = {
  init: function () {
    this.appendEndRowInput('NAME')
      .appendField('a')
      .appendField(new Blockly.FieldTextInput('default'), 'input');
    this.setPreviousStatement(true, null);
    this.setNextStatement(true, null);
    this.setTooltip('');
    this.setHelpUrl('');
    this.setColour(225);
  }
};

Blockly.common.defineBlocks({ a_block: a_block });

javascript.javascriptGenerator.forBlock['a_block'] = function (block) {
  const text_input = block.getFieldValue('input');
  const code = `a(${text_input})`;
  return code;
}

const b_block = {
  init: function () {
    this.appendEndRowInput('NAME')
      .appendField('b')
      .appendField(new Blockly.FieldTextInput('default'), 'input')
      .appendField('b');
    this.setPreviousStatement(true, null);
    this.setNextStatement(true, null);
    this.setTooltip('');
    this.setHelpUrl('');
    this.setColour(225);
  }
};

Blockly.common.defineBlocks({ b_block: b_block });

javascript.javascriptGenerator.forBlock['b_block'] = function (block) {
  const input_text = block.getFieldValue('input');
  const code = `b(${input_text})`;
  return code;
}

const start_block = {
  init: function () {
    this.appendEndRowInput('NAME')
      .appendField('start');
    this.setNextStatement(true, null);
    this.setTooltip('');
    this.setHelpUrl('');
    this.setColour(225);
  }
};

Blockly.common.defineBlocks({ start_block: start_block });

javascript.javascriptGenerator.forBlock['start_block'] = function () {
  return "";
}
