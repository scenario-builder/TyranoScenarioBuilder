import * as Blockly from 'blockly';

const workspace = Blockly.inject('blocklyDiv',
  { toolbox: {
    "kind": "categoryToolbox",
    "contents": [
      {
        "kind": "category",
        "name": "core",
        "contents": [
          {
            "kind": "block",
            "type": "a_block"
          },
          {
            "kind": "block",
            "type": "b_block"
          },
          {
            "kind": "block",
            "type": "start_block"
          }
        ]
      }
    ]
  } });

function exec() {
  Blockly.JavaScript.addReservedWords('code');
  var code = Blockly.JavaScript.workspaceToCode(workspace);
  workspace.addChangeListener(Blockly.Events.disableOrphans);
  try {
    alert(code);
  } catch (e) {
    alert(e);
  }
}