use crate::bin_serializable;
use crate::models::index::checksum_index::ChecksumIndex;
use crate::models::index::index_node::IndexNode;

bin_serializable!(IndexNode);
bin_serializable!(ChecksumIndex);
