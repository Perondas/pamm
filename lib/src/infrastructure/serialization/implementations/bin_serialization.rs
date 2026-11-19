use crate::bin_serializable;
use crate::domain::index::checksum_index::ChecksumIndex;
use crate::domain::index::index_node::IndexNode;

bin_serializable!(IndexNode);
bin_serializable!(ChecksumIndex);
