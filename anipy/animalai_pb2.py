# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: animalai.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x0e\x61nimalai.proto\x12\x08\x61nimalai\".\n\x08\x42oardReq\x12\r\n\x05\x63\x65lls\x18\x01 \x03(\x02\x12\x13\n\x0blegal_moves\x18\x02 \x03(\x04\"\x1b\n\tActionRep\x12\x0e\n\x06\x61\x63tion\x18\x01 \x01(\x04\x32\x44\n\x08\x41iPlayer\x12\x38\n\x0bThinkAction\x12\x12.animalai.BoardReq\x1a\x13.animalai.ActionRep\"\x00\x62\x06proto3')



_BOARDREQ = DESCRIPTOR.message_types_by_name['BoardReq']
_ACTIONREP = DESCRIPTOR.message_types_by_name['ActionRep']
BoardReq = _reflection.GeneratedProtocolMessageType('BoardReq', (_message.Message,), {
  'DESCRIPTOR' : _BOARDREQ,
  '__module__' : 'animalai_pb2'
  # @@protoc_insertion_point(class_scope:animalai.BoardReq)
  })
_sym_db.RegisterMessage(BoardReq)

ActionRep = _reflection.GeneratedProtocolMessageType('ActionRep', (_message.Message,), {
  'DESCRIPTOR' : _ACTIONREP,
  '__module__' : 'animalai_pb2'
  # @@protoc_insertion_point(class_scope:animalai.ActionRep)
  })
_sym_db.RegisterMessage(ActionRep)

_AIPLAYER = DESCRIPTOR.services_by_name['AiPlayer']
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _BOARDREQ._serialized_start=28
  _BOARDREQ._serialized_end=74
  _ACTIONREP._serialized_start=76
  _ACTIONREP._serialized_end=103
  _AIPLAYER._serialized_start=105
  _AIPLAYER._serialized_end=173
# @@protoc_insertion_point(module_scope)
