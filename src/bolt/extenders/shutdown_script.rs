// LNP/BP Core Library implementing LNPBP specifications & standards
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use crate::bolt::ExtensionId;
use crate::{channel, ChannelExtension, Extension};
use p2p::legacy::Messages;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ShutdownScript {}

impl Extension for ShutdownScript {
    type Identity = ExtensionId;

    #[inline]
    fn identity(&self) -> Self::Identity {
        ExtensionId::ShutdownScript
    }

    #[inline]
    fn update_from_peer(&mut self, _: &Messages) -> Result<(), channel::Error> {
        // TODO: Implement
        Ok(())
    }

    #[inline]
    fn extension_state(&self) -> Box<dyn channel::State> {
        Box::new(())
    }
}

impl ChannelExtension for ShutdownScript {
    #[inline]
    fn channel_state(&self) -> Box<dyn channel::State> {
        Box::new(())
    }

    #[inline]
    fn apply(
        &mut self,
        _tx_graph: &mut channel::TxGraph,
    ) -> Result<(), channel::Error> {
        todo!("implement shutdown script")
    }
}