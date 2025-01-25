var sidebarBtn = document.querySelector('.sidebar-toggle');
var sidebar = document.querySelector('.sidebar');

if(sidebarBtn && sidebar) {
  sidebarBtn.addEventListener('click', function() {
    sidebarBtn.classList.toggle('sidebar-closed');
    sidebar.classList.toggle('sidebar-closed');
  })
}

console.log(Blockly);

const a_block = {
  init: function () {
    this.appendEndRowInput('NAME')
      .appendField('a')
      .appendField(new Blockly.FieldTextInput('default'), 'b');
    this.setPreviousStatement(true, null);
    this.setNextStatement(true, null);
    this.setTooltip('');
    this.setHelpUrl('');
    this.setColour(225);
  }
};

const b_block = {
  init: function () {
    this.appendEndRowInput('NAME')
      .appendField('b')
      .appendField(new Blockly.FieldTextInput('default'), 'b')
      .appendField('b');
    this.setPreviousStatement(true, null);
    this.setNextStatement(true, null);
    this.setTooltip('');
    this.setHelpUrl('');
    this.setColour(225);
  }
};

Blockly.common.defineBlocks({ a_block: a_block });

Blockly.common.defineBlocks({ b_block: b_block });

